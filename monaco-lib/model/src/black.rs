use std::any::Any;
use crate::model::Model;
use data_cube::data_cube::Cube;
use serde::{Serialize, Deserialize};
use macros::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Black
{
    pub name: String,
    pub r:f64,
    pub sigmas: Vec<(f64,f64)>,
    pub initial_value: f64
}

impl Model for Black
{
    fn as_any(&self) -> &dyn Any { self }

    fn init(&mut self) -> () { }
    fn get_name(&self) -> String
    {
        return self.name.clone();
    }
    fn get_type(&self) -> &str
    {
        return "black";
    }
    fn get_number_of_variables(&self) -> usize
    {
        return 1;
    }
    fn get_number_of_outputs(&self) -> usize
    {
        return 1;
    }

    #[allow(non_snake_case)]
    fn populate_factors(&self,start_raw: usize, raw_factors:&Cube, start:usize, factors:&mut Cube) -> ()
    {
        for s in 0..factors.num_scenarios
        {
            for dt_idx in 0..factors.dates.len()
            {
                let t:f64=factors.dates[dt_idx];
                if t==0.0
                {
                    factors.set_item(s, start, 0, self.initial_value).unwrap();
                }
                else
                {
                    let prev_t:f64=if dt_idx==0 {0.0} else {factors.dates[dt_idx - 1]};  
                    let delta_t:f64 = t - prev_t;

                    let sigma:f64=math::math::interpolate(&self.sigmas, t);
                    let dW=raw_factors.get_item(s, start_raw, dt_idx).unwrap();
                    let prev:f64=if dt_idx==0 { self.initial_value } else { factors.get_item(s, start, dt_idx-1).unwrap() };
                    let v:f64=prev+prev*(self.r*delta_t+sigma*dW*f64::sqrt(delta_t));

                    //debug!(format!("black-Populate factors -> name: {0}, s: {1}, dt_idx: {2}, delta_t: {3}, sigma:{4}, dW: {5}, prev: {6}, v: {7}, r: {8}",self.name,s,dt_idx,delta_t,sigma,dW,prev,v,self.r));

                    factors.set_item(s, start, dt_idx, v).unwrap();
                }
            }
        }
    }
    fn get_output_values(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64) -> Result<Vec<f64>,String>
    {
        //TODO: Martingale interpolation
        let res=cube.get_item_interp(scenario,start_pos,date,true);
        let r:f64=match res {
            Ok(r)     =>   r.2,
            Err(e)    =>   { return Err(format!("Hw1f - {}{}","Error: ",&e)) },
        };
        return Ok(vec![r]);
    }
    fn get_value(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64, _term:f64) -> Result<f64,String>
    {
        let res=self.get_output_values(start_pos,cube,raw_start_pos,raw_cube,scenario,date);
        let v:f64=match res {
            Ok(v)     =>   v[0],
            Err(e)    =>   { return Err(format!("Black - {}{}","Error: ",&e)) },
        };
        return Ok(v);
    }
}