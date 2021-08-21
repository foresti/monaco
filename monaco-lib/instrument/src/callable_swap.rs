use macros::debug;
use crate::instrument::Instrument;
use crate::vanilla_swap::VanillaSwap;
use crate::lsm::*;
use model::live_model::LiveModel;
use data_cube::data_cube::Cube;
use curve::curve::Curve;
use curve::curve::IrCurve;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use logger::Logger;


/// Callable swap implementation
/// (vanilla swap + call dates)
#[derive(Serialize, Deserialize)]
pub struct CallableSwap
{
    pub name: String,
    pub exposure_discount_model_name:String,
    pub underlying:VanillaSwap,
    pub call_dates:Vec<f64>
}

impl CallableSwap
{
    ///Add a 'rich' (date,callable_flag) date to an vec of rich dates
    /// If a date is already present, then xor the callability flags
    fn add_rich_date(rich_dates:&mut Vec<(bool,f64)>,is_call_date:bool,date:f64) -> ()
    {
        let tolerance:f64=0.00001;
        for rd in 0..rich_dates.len()
        {
            if (rich_dates[rd].1-date).abs()<tolerance
            {
                rich_dates[rd].0=is_call_date;
                return;
            }
        }
        rich_dates.push((is_call_date,date));
    }
}

impl Instrument for CallableSwap
{
    fn get_name(&self) -> String
    {
        return self.name.clone();
    }

    fn compute_values(&self,start:usize,result_cube:&mut Cube,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> (Vec<Vec<(f64,f64)>>,Cube)
    {
        let date_threshold=0.001;
        let exp_dsc_model=match live_models.get(&self.exposure_discount_model_name)
        {
            Some(leg)   =>  leg,
            None        =>  panic!("Instrument {} refers to non-existent model ({})!",&self.name,&self.exposure_discount_model_name)
        };

        //Remember: swaps must have at least one leg
        let maturity_date=self.underlying.legs.iter().map(|leg| *leg.payment_dates.last().unwrap()).fold(f64::NEG_INFINITY, f64::max); 
        logger.log(format!("callable-swap|compute_values -> maturity: {}",maturity_date),"instrument");
        //Create instrument values cube: the dates are the union of the dates in the result cube + call dates+ maturity date
        //A rich date is a tuple (is_call_date,date)
        let mut rich_dates:Vec<(bool,f64)>=Vec::new();
        for dt_idx in 0..result_cube.dates.len()
        {
            if (maturity_date-result_cube.dates[dt_idx])>date_threshold
            {
                rich_dates.push((false,result_cube.dates[dt_idx]));
            }
        }
        //For callable swaps the maturity date does not affect callability (i.e. value is always 0 at maturity)
        CallableSwap::add_rich_date(&mut rich_dates,false,maturity_date);

        //Add call dates
        for call_dt_idx in 0..self.call_dates.len() 
        {
            CallableSwap::add_rich_date(&mut rich_dates,true,self.call_dates[call_dt_idx]);
        }

        rich_dates.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
        
        for d in 0..rich_dates.len()
        {
            logger.log(format!("callable-swap|compute_values -> rich_dates ({}): {}/{}",d,rich_dates[d].0,rich_dates[d].1),"instrument");
        }

        let dates=rich_dates.iter().map(|d| d.1).collect();
        let exercise_flags=rich_dates.iter().map(|d| d.0).collect();
        let mut instrument_values_cube:Cube=Cube::make_empty_cube(dates, result_cube.num_scenarios, 1);

        logger.log("callable-swap|compute_values -> Starting lsm...","instrument");
        let (cashflows,exercise_cube)=compute_lsm_values(
            &mut instrument_values_cube,
            &live_models,
            &exercise_flags,
            &mut |date, live_models| self.underlying.get_models_variables_values(date, &live_models),
            &mut |scenario,date,live_models| -self.underlying.get_value(scenario,date,&live_models),
            // &mut |scenario,min_date,max_date,live_models| {let mut cf=self.underlying.get_cashflows(scenario,min_date,max_date,&live_models); for i in 0..cf.len() {cf[i].1=-cf[i].1} return cf },
            &mut |scenario,min_date,max_date,live_models| self.underlying.get_cashflows(scenario,min_date,max_date,&live_models).iter().map(|cf| (cf.0,-cf.1)).collect(),
            &exp_dsc_model,
            &logger
        );
        logger.log("callable-swap|compute_values -> lsm done.","instrument");

        for s in 0..result_cube.num_scenarios
        {
            for dt_idx in 0..result_cube.dates.len()
            {
                let v=instrument_values_cube.get_item_interp(s, 0, result_cube.dates[dt_idx], true).unwrap().2;
                let _=result_cube.set_item(s, start, dt_idx, v);
            }
        }
        return (cashflows,exercise_cube);
    }
}