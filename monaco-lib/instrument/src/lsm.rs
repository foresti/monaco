use macros::debug;
use data_cube::data_cube::Cube;
use model::live_model::LiveModel;
use math::matrix::*;
use math::linear_regressor::*;
use std::collections::HashMap;
use logger::Logger;

/// LSM calculator
/// 
/// # Arguments
/// 
/// * `instrument_values_cube` - Cube where to write the calculated values (contains only one series)
/// * `live_models` - Dictionary of live models to be used for calculations
/// * `exercise_flags` - Vec of flags that indicate the callability of a date (refers the dates of 'instrument_values_cube')
/// * `f_models_variables_values` - Function that returns the variable values for a date (variables are in matrix order (rows are scenarios, cols are variable order))
/// * `f_exercise_value` - Function that calculates the exercise value for a scenario and date
/// * `f_cashflows` - Function that returns the *effective* cashflows for a scenario between two dates, with: (start_date,end_date]
/// * `discount_model` - Model used to discount values and cashflows to the evaluation dates
/// 
/// # Remarks
/// 
///  Last date of 'instrument_values_cube' is maturity date, previous dates are (evaluation dates U call/exercise dates)
pub fn compute_lsm_values   (   
                                instrument_values_cube:&mut Cube,
                                live_models:&HashMap<String,LiveModel>,
                                exercise_flags:&Vec<bool>,
                                f_models_variables_values:&mut impl FnMut(f64,&HashMap<String,LiveModel>) -> Vec<f64>,
                                f_exercise_value:&mut impl FnMut(usize,f64,&HashMap<String,LiveModel>) -> f64,
                                f_cashflows:&mut impl FnMut(usize,f64,f64,&HashMap<String,LiveModel>) -> Vec<(f64,f64)>,
                                discount_model:&LiveModel,
                                logger:&Logger
                            ) -> (Vec<Vec<(f64,f64)>>,Cube)
{
    let mut ret_cashflows:Vec<Vec<(f64,f64)>>=Vec::new();
    //Prepare the exercise cube (it contains 1 for exercise dates and 0 for non-exercise)
    let mut exercise_values_cube:Cube=Cube::make_empty_cube(instrument_values_cube.dates.clone(), instrument_values_cube.num_scenarios, 1);
    for dt_inv_idx in 0..instrument_values_cube.dates.len()
    {
        //Index is inverted for American Monte Carlo backward pass
        let dt_idx=instrument_values_cube.dates.len()-dt_inv_idx-1;
        let dt=instrument_values_cube.dates[dt_idx];

        let mut date_cashflows:Vec<Vec<(f64,f64)>>=Vec::new();

        //The final date in the instrument_values_cube is the maturity date (where the value is 0 by convention)
        if dt_inv_idx==0
        {
            logger.log(format!("lsm -> Setting maturity values (date:{}) ...",dt),"lsm");
            for s in 0..instrument_values_cube.num_scenarios
            {
                let _=instrument_values_cube.set_item(s, 0, dt_idx, 0.0);
            }
        }
        else
        {
            logger.log(format!("lsm -> date: {}",dt),"lsm");
            let mut next_values:Vec<f64>=vec![0.0;instrument_values_cube.num_scenarios];
            let variable_values:Vec<f64>=f_models_variables_values(dt,&live_models);
            let num_variables=variable_values.len()/instrument_values_cube.num_scenarios;
            logger.log(format!("lsm -> num_variables: {}",num_variables),"lsm");

            let mut exercise_values:Vec<f64>=vec![0.0;instrument_values_cube.num_scenarios];
            let mut num_paths_itm:usize=0;
            for s in 0..instrument_values_cube.num_scenarios
            {
                let exercise_value=f_exercise_value(s,dt,&live_models);
                exercise_values[s]=exercise_value;
                if exercise_value>0.0
                {
                    num_paths_itm+=1;
                }
            }
            logger.log(format!("lsm -> num_paths_itm: {}",num_paths_itm),"lsm");

            for s in 0..instrument_values_cube.num_scenarios
            {
                //Find date of previous exercise/maturity
                let mut next_dt_idx=dt_idx;
                loop
                {
                        next_dt_idx+=1;
                        if next_dt_idx==instrument_values_cube.dates.len()-1 || exercise_values_cube.get_item(s, 0, next_dt_idx).unwrap()>0.0
                        {
                            break;
                        }
                }
                let next_dt=instrument_values_cube.dates[next_dt_idx];

                //logger.log(format!("lsm -> next_dt_idx: {}, next_dt: {}, dt_idx: {}, dt: {}",next_dt_idx,next_dt,dt_idx,dt),"lsm");

                let mut v=instrument_values_cube.get_item(s, 0, next_dt_idx).unwrap();
                let r=discount_model.get_value(s, dt, next_dt-dt).unwrap();
                v=v*(-r*(next_dt-dt)).exp();
                next_values[s]=v;

                let cashflows=f_cashflows(s,dt,next_dt,&live_models);
                for c in 0..cashflows.len()
                {
                    let r=discount_model.get_value(s, dt, cashflows[c].0-dt).unwrap();
                    let df=(-r*(cashflows[c].0-dt)).exp();
                    next_values[s]+=cashflows[c].1*df;
                }
                date_cashflows.push(cashflows);
            }

            //Trim variable values and next values to show only in-the-money paths
            let mut itm_variable_values:Vec<f64>=vec![0.0;num_paths_itm*num_variables];
            let mut itm_next_values:Vec<f64>=vec![0.0;num_paths_itm];
            let mut itm_s:usize=0;
            for s in 0..exercise_values.len()
            {
                if exercise_values[s]>0.0
                {
                    for v in 0..num_variables
                    {
                        itm_variable_values[itm_s]=variable_values[s*num_variables+v];
                    }
                    itm_next_values[itm_s]=next_values[s];
                    itm_s+=1;
                }
            }

            //Get regression parameters for next_values over variable values
            let mut itm_variable_values_squared:Vec<f64>=vec![0.0;itm_variable_values.len()];
            for i in 0..itm_variable_values.len()
            {
                itm_variable_values_squared[i]=itm_variable_values[i]*itm_variable_values[i];
            }
            let regression_variables=horizontal_add(&itm_variable_values,&itm_variable_values_squared,instrument_values_cube.num_scenarios);
            
            let x_str=math::matrix::display_matrix(&itm_variable_values,itm_next_values.len(),num_variables,false);
            let y_str=math::matrix::display_matrix(&itm_next_values,itm_next_values.len(),1,false);
            logger.log(format!("lsm|regression -> date: {} - x: {} - y: {}",dt,x_str,y_str),"lsm");
            let regressor=LinearRegressor::create_regressor(&regression_variables,&itm_next_values);
            
            //Pricing regressor
            let mut variable_values_squared:Vec<f64>=vec![0.0;variable_values.len()];
            for i in 0..variable_values.len()
            {
                variable_values_squared[i]=variable_values[i]*variable_values[i];
            }
            let pricing_regression_variables=horizontal_add(&variable_values,&variable_values_squared,instrument_values_cube.num_scenarios);
            let pricing_regressor=LinearRegressor::create_regressor(&pricing_regression_variables,&next_values);

            for s in 0..instrument_values_cube.num_scenarios
            {
                //Calculate value from regression
                let regression_value:f64=match regressor
                {
                    Ok(ref r)   =>  {
                                        let mut x:Vec<f64>=vec![0.0;num_variables*2];
                                        for i in 0..num_variables
                                        {
                                            let val=variable_values[(s*num_variables)+i];
                                            x[i]=val;
                                            x[num_variables+i]=val*val;
                                        }
                                        r.get_value(&x)
                                },
                    Err(_)  =>  0.0
                };
                if exercise_flags[dt_idx]
                {
                    //let exercise_value=f_exercise_value(s,dt,&live_models);
                    let exercise_value:f64=exercise_values[s];
                    if num_paths_itm>0 && exercise_value>regression_value
                    {
                        logger.log(format!("lsm -> early exercise - scenario: {}, dt_idx: {}, dt: {}, exercise_value: {}, regression_value: {}",s,dt_idx,dt,exercise_value,regression_value),"lsm");
                        let _=exercise_values_cube.set_item(s, 0, dt_idx, 1.0);
                        for d in dt_idx+1..exercise_values_cube.dates.len()
                        {
                            let _=exercise_values_cube.set_item(s, 0, d, 0.0);
                            let _=instrument_values_cube.set_item(s, 0, d, 0.0);
                        }
                        let _=instrument_values_cube.set_item(s, 0, dt_idx, exercise_value);
                    }
                    else
                    {
                        logger.log(format!("lsm -> no early exercise - scenario: {}, dt_idx: {}, dt: {}, exercise_value: {}, regression_value: {}",s,dt_idx,dt,exercise_value,regression_value),"lsm");
                        let _=instrument_values_cube.set_item(s, 0, dt_idx, regression_value);
                    }
                }
                else
                {
                    let pricing_regression_value:f64=match pricing_regressor
                    {
                        Ok(ref r)   =>  {
                                            let mut x:Vec<f64>=vec![0.0;num_variables*2];
                                            for i in 0..num_variables
                                            {
                                                let val=variable_values[(s*num_variables)+i];
                                                x[i]=val;
                                                x[num_variables+i]=val*val;
                                            }
                                            r.get_value(&x)
                                    },
                        Err(_)  =>  0.0
                    };
                    logger.log(format!("lsm -> not an exercise date - scenario: {}, dt_idx: {}, dt: {}, pricing_regression_value: {}",s,dt_idx,dt,pricing_regression_value),"lsm");
                    let _=instrument_values_cube.set_item(s, 0, dt_idx, pricing_regression_value);
                }
            }
        }
        if date_cashflows.len()!=0
        {
            if ret_cashflows.len()==0
            {
                for _s in 0..instrument_values_cube.num_scenarios
                {
                    ret_cashflows.push(Vec::new());
                }
            }
            for s in 0..instrument_values_cube.num_scenarios
            {
                ret_cashflows[s].append(&mut date_cashflows[s]);
            }
        }
    }

    //Make sure that the cashflows are in the correct order
    for s in 0..instrument_values_cube.num_scenarios
    {
        ret_cashflows[s].sort_by(|a, b| if a.0>b.0 {std::cmp::Ordering::Greater} else {if a.0<b.0 {std::cmp::Ordering::Less} else {std::cmp::Ordering::Equal}});
    }
    return (ret_cashflows,exercise_values_cube);
}