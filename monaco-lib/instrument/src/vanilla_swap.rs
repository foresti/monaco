//use std::any::Any;
use crate::instrument::Instrument;
use model::live_model::LiveModel;
use data_cube::data_cube::Cube;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
//use macros::debug;
use logger::Logger;

#[derive(Serialize, Deserialize)]
pub struct SwapLeg
{
    pub notional:f64,
    pub pay_or_receive:String,
    pub discount_model_name:String,
    pub projection_model_name:String,
    pub fx_model_name:String,
    pub payment_dates:Vec<f64>,
    pub is_fixed:bool,
    pub fixed_values:Vec<f64>
}

#[derive(Serialize, Deserialize)]
pub struct VanillaSwap
{
    pub name: String,
    pub legs:Vec<SwapLeg>
}

impl VanillaSwap
{
    //To be removed
    // fn test(&self,ex_fl:&Vec<bool>,test_cube:&mut Cube,live_models:&HashMap<String,LiveModel>,discount_model:&LiveModel) -> ()
    // {
    //     compute_lsm_values  (
    //                             test_cube,
    //                             live_models,
    //                             ex_fl,
    //                             &mut |date, live_models| self.get_models_variables_values(date, &live_models),
    //                             &mut |scenario,date,live_models| self.get_value(scenario,date,&live_models),
    //                             &mut |scenario,min_date,max_date,live_models| self.get_cashflows(scenario,min_date,max_date,&live_models),
    //                             discount_model
    //                         );
    // }
    pub fn get_models_variables_values(&self,date:f64,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> Vec<f64>
    {
        let mut num_variables:usize=0;
        let models=self.get_live_models_map(&live_models);

        let mut num_scenarios:usize=0;
        for (_k,v) in models.iter()
        {
            if num_scenarios==0
            {
                num_scenarios=v.cube.num_scenarios;
            }
            logger.log(format!("vanilla-swap|get_models_variables_values -> model: {}, number of variables: {}",v.model.get_name(),v.model.get_number_of_variables()),"instrument");
            num_variables+=v.model.get_number_of_variables();
        }

        logger.log(format!("vanilla-swap|get_models_variables_values -> models count: {}, number of variables: {}",models.len(),num_variables),"instrument");
        let mut variables_values:Vec<f64>=vec![0.0;num_variables*num_scenarios];

        for s in 0..num_scenarios
        {
            let mut num_var:usize=0;
            for (_k,v) in models.iter()
            {
                let values:Vec<f64>=v.get_variable_values(s, date,&logger);
                for i in 0..values.len() { variables_values[s*num_variables+num_var+i]=values[i]; }
                num_var+=values.len();
            }
        }
        logger.log(format!("vanilla-swap|get_models_variables_values -> Done"),"instrument");
        return variables_values;
    }

    fn get_direct_value_for_leg(&self,leg_no:usize,scenario:usize,date:f64,models:&Vec<(&model::live_model::LiveModel,&model::live_model::LiveModel,&model::live_model::LiveModel)>,logger:&Logger) -> f64
    {
        let mut v:f64=0.0;
        for t in 0..self.legs[leg_no].payment_dates.len()
        {
            if self.legs[leg_no].payment_dates[t]>date
            {
                let cpn_end=self.legs[leg_no].payment_dates[t];
                let cpn_start=if t==0 { 0.0 } else { self.legs[leg_no].payment_dates[t-1] };
                let cpn_period=if t==0 { 0.0 } else { self.legs[leg_no].payment_dates[t]-self.legs[leg_no].payment_dates[t-1] };
                let cpn_time=self.legs[leg_no].payment_dates[t]-date;

                let mut leg_payment=if t==0||self.legs[leg_no].is_fixed
                {
                    self.legs[leg_no].fixed_values[t]
                }
                else
                {
                    let fwd:f64;
                    //Debug
                    //let fwd_test1=model::live_model::get_fwd_rate(&models[leg_no].1, scenario, date, cpn_start, cpn_end);
                    //let fwd_test2=models[leg_no].1.get_value(scenario,cpn_start,cpn_end-cpn_start).unwrap();
                    //End debug
                    if cpn_start>date
                    {
                        fwd=model::live_model::get_fwd_rate(&models[leg_no].1, scenario, date, cpn_start, cpn_end,&logger);
                    }
                    else
                    {
                        fwd=models[leg_no].1.get_value(scenario,cpn_start,cpn_end-cpn_start,&logger).unwrap()
                    }                   
                    fwd*cpn_period+self.legs[leg_no].fixed_values[t]
                    //fwd+self.legs[leg_no].fixed_values[t]
                };
                leg_payment*=self.legs[leg_no].notional;
                let leg_df=(-cpn_time*models[leg_no].0.get_value(scenario, date, cpn_time,&logger).unwrap()).exp();
                v=if self.legs[leg_no].pay_or_receive=="pay" { v-leg_payment*leg_df } else { v+leg_payment*leg_df };
                // debug!(format!("vanilla-swap|get_direct_value_for_leg -> name: {}, date: {}, scenario: {}, t: {}, is_fixed: {}, cpn_start: {}, cpn_end: {}, cpn_period: {}, cpn_time: {}, leg_payment: {}, leg_df: {}",
                // self.name,
                // date,
                // scenario,
                // t,
                // self.legs[leg_no].is_fixed,
                // cpn_start,
                // cpn_end,
                // cpn_period,
                // cpn_time,
                // leg_payment,
                // leg_df));
            }
        }
        let leg_fx=models[leg_no].2.get_value(scenario, date,0.0,&logger).unwrap();

        let v=v*leg_fx;
        return v;
    }

    fn get_direct_value(&self,scenario:usize,date:f64,models:&Vec<(&model::live_model::LiveModel,&model::live_model::LiveModel,&model::live_model::LiveModel)>,logger:&Logger) -> f64
    {
        let mut v:f64=0.0;
        for l in 0..self.legs.len()
        {
            let leg_value=self.get_direct_value_for_leg(l,scenario,date,models,&logger);
            v+=leg_value;
            // debug!(format!("vanilla-swap|get_direct_value -> name: {}, date: {}, leg: {}, leg_value: {}, v: {}",
            // self.name,
            // date,
            // l,
            // leg_value,
            // v
            // ));
        }
        return v;
    }

    pub fn get_value(&self,scenario:usize,date:f64,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> f64
    {
        let models=self.get_live_models(&live_models);
        return self.get_direct_value(scenario,date,&models,&logger);
    }

    fn get_cashflows_for_leg(&self,leg_no:usize,scenario:usize,min_date:f64,max_date:f64,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> Vec<(f64,f64)>
    {
        let mut cashflows:Vec<(f64,f64)>=Vec::new();
        let models=self.get_live_models(&live_models);
        
        for t in 0..self.legs[leg_no].payment_dates.len()
        {
            if self.legs[leg_no].payment_dates[t]>min_date&&self.legs[leg_no].payment_dates[t]<=max_date
            {
                let cpn_start=if t==0 { 0.0 } else { self.legs[leg_no].payment_dates[t-1] };
                let cpn_period=if t==0 { 0.0 } else { self.legs[leg_no].payment_dates[t]-self.legs[leg_no].payment_dates[t-1] };

                let mut leg_payment=if t==0||self.legs[leg_no].is_fixed
                {
                    self.legs[leg_no].fixed_values[t]
                }
                else
                {
                    models[leg_no].1.get_value(scenario, cpn_start, cpn_period,&logger).unwrap()*cpn_period+self.legs[leg_no].fixed_values[t]
                };
                leg_payment*=self.legs[leg_no].notional;
                if self.legs[leg_no].pay_or_receive=="pay" { leg_payment=-leg_payment; } 
                let leg_fx=models[leg_no].2.get_value(scenario,self.legs[leg_no].payment_dates[t],0.0,&logger).unwrap();
                let global_ccy_leg_payment=leg_payment*leg_fx;
                cashflows.push((self.legs[leg_no].payment_dates[t],global_ccy_leg_payment));
            }
        }
        
        return cashflows;
    }

    pub fn get_cashflows(&self,scenario:usize,min_date:f64,max_date:f64,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> Vec<(f64,f64)>
    {
        let mut cashflows:Vec<(f64,f64)>=Vec::new();
        for l in 0..self.legs.len()
        {
            let mut cf=self.get_cashflows_for_leg(l, scenario, min_date, max_date, live_models,&logger);
            cashflows.append(&mut cf);
        }
        return cashflows;
    }

    pub fn get_live_models_map<'a>(&'a self,live_models:&'a HashMap<String,LiveModel<'a>>) -> HashMap<String,&'a LiveModel<'a>> //Vec<(&'a model::live_model::LiveModel,&'a model::live_model::LiveModel,&'a model::live_model::LiveModel)>
    {
        let mut models:HashMap<String,&'a LiveModel<'a>>=HashMap::new();
        for l in 0..self.legs.len()
        {
            let leg_discount_model=match live_models.get(&self.legs[l].discount_model_name)
            {
                Some(leg)   =>  leg,
                None        =>  panic!("Instrument {} (leg {}) refers to non-existent model ({})!",&self.name,l,&self.legs[l].discount_model_name)
            };
            let leg_projection_model=match live_models.get(&self.legs[l].projection_model_name)
            {
                Some(leg)   =>  leg,
                None        =>  panic!("Instrument {} (leg {}) refers to non-existent model ({})!",&self.name,l,&self.legs[l].projection_model_name)
            };
            let leg_fx_model=match live_models.get(&self.legs[l].fx_model_name)
            {
                Some(leg)   =>  leg,
                None        =>  panic!("Instrument {} (leg {}) refers to non-existent model ({})!",&self.name,l,&self.legs[l].fx_model_name)
            };
            models.insert(leg_discount_model.model.get_name(),leg_discount_model);
            models.insert(leg_projection_model.model.get_name(),leg_projection_model);
            models.insert(leg_fx_model.model.get_name(),leg_fx_model);
        }

        return models;
    }

    pub fn get_live_models<'a>(&'a self,live_models:&'a HashMap<String,LiveModel<'a>>) -> Vec<(&'a model::live_model::LiveModel,&'a model::live_model::LiveModel,&'a model::live_model::LiveModel)>
    {
        let mut models:Vec<(&'a model::live_model::LiveModel,&'a model::live_model::LiveModel,&'a model::live_model::LiveModel)>=Vec::new();
        for l in 0..self.legs.len()
        {
            let leg_discount_model=match live_models.get(&self.legs[l].discount_model_name)
            {
                Some(leg)   =>  leg,
                None        =>  panic!("Instrument {} (leg {}) refers to non-existent model ({})!",&self.name,l,&self.legs[l].discount_model_name)
            };
            let leg_projection_model=match live_models.get(&self.legs[l].projection_model_name)
            {
                Some(leg)   =>  leg,
                None        =>  panic!("Instrument {} (leg {}) refers to non-existent model ({})!",&self.name,l,&self.legs[l].projection_model_name)
            };
            let leg_fx_model=match live_models.get(&self.legs[l].fx_model_name)
            {
                Some(leg)   =>  leg,
                None        =>  panic!("Instrument {} (leg {}) refers to non-existent model ({})!",&self.name,l,&self.legs[l].fx_model_name)
            };
            models.push((leg_discount_model,leg_projection_model,leg_fx_model));
        }

        return models;
    }
}

impl Instrument for VanillaSwap
{
    fn get_name(&self) -> String
    {
        return self.name.clone();
    }

    fn compute_values(&self,start:usize,result_cube:&mut Cube,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> (Vec<Vec<(f64,f64)>>,Cube)
    {
        let models=self.get_live_models(&live_models);
        let mut cashflows:Vec<Vec<(f64,f64)>>=Vec::new();
        for s in 0..result_cube.num_scenarios
        {
            let mut scenario_cashflows=self.get_cashflows(s, 0.0, 9999.0, &live_models,&logger);
            cashflows.push(Vec::new());
            cashflows[s].append(&mut scenario_cashflows);

            for dt_idx in 0..result_cube.dates.len()
            {
                let date=result_cube.dates[dt_idx];
                let v=self.get_direct_value(s,date,&models,&logger);
                logger.log(format!("vanilla-swap|compute_values -> name: {}, s: {}, dt_idx: {}, date: {}, v: {}",self.name,s,dt_idx,date,v),"instrument");
                let _=result_cube.set_item(s, start, dt_idx, v);
            }
        }

        let exercise_cube=Cube::make_empty_cube(vec![0.0], 1, 1);
        return (cashflows,exercise_cube);
    }
}