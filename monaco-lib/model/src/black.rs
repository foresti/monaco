use std::any::Any;
use crate::model::OutputInterpolation;
use crate::model::Model;
use data_cube::data_cube::Cube;
use serde::{Serialize, Deserialize};
//use macros::debug;
use logger::Logger;

#[derive(Serialize, Deserialize, Debug)]
pub struct Black
{
    pub name: String,
    pub interpolation: OutputInterpolation,
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
    fn populate_factors(&self,start_raw: usize, raw_factors:&Cube, start:usize, factors:&mut Cube,logger:&Logger) -> ()
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

                    logger.log(format!("black|populate_factors -> name: {0}, s: {1}, dt_idx: {2}, delta_t: {3}, sigma:{4}, dW: {5}, prev: {6}, v: {7}, r: {8}",self.name,s,dt_idx,delta_t,sigma,dW,prev,v,self.r),"model");

                    factors.set_item(s, start, dt_idx, v).unwrap();
                }
            }
        }
    }
    #[allow(non_snake_case)]
    fn get_output_values(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64,logger:&Logger) -> Result<Vec<f64>,String>
    {
        let martingale_interp:bool=match &self.interpolation
        {
            OutputInterpolation::Linear => false,
            OutputInterpolation::Martingale => true
        };
        let prev_v_res=cube.get_item_last(scenario,start_pos,date);
        let (prev_dt_idx,prev_v)=match prev_v_res
        {
            Ok((p,q)) => (p,q),
            Err(e) => return Err(format!("Black - {}{}","Error: ",&e)),
        };
        if date>=cube.dates[cube.dates.len()-1]
        {
            return Ok(vec![prev_v]);
        }
        else
        {
            if martingale_interp
            {
                //Martingale interpolation
                let prev_dt_res=cube.get_date(prev_dt_idx);
                let prev_dt=match prev_dt_res
                {
                    Ok(v) => v,
                    Err(e) => return Err(format!("Black - {}{}","Error: ",&e)),
                };
                let next_dt_idx=prev_dt_idx+1;
                let next_dt_res=cube.get_date(next_dt_idx);
                let next_dt=match next_dt_res
                {
                    Ok(v) => v,
                    Err(e) => return Err(format!("Hw1f - {}{}","Error: ",&e)),
                };

                let (prev_X_pos,_prev_X)=match raw_cube.get_item_last(scenario,raw_start_pos,date)
                {
                    Ok((p,q)) => (p,q),
                    Err(e) => return Err(format!("Black - {}{}","Error: ",&e)),
                };
                let next_X=match raw_cube.get_item(scenario,raw_start_pos,prev_X_pos+1)
                {
                    Ok(v) => v,
                    Err(e) =>  return Err(format!("Black - {}{}","Error: ",&e)),
                };
                let prev_sigma=math::math::interpolate(&self.sigmas, prev_dt);

                let delta_t=date-prev_dt;

                let b_wgt:f64=((date-prev_dt)/(next_dt-prev_dt)).sqrt();

                //Sigma is defined at the beginning of each time step
                let D=prev_v*(prev_sigma*(delta_t)*((b_wgt*next_X)-0.5*(prev_sigma*prev_sigma*(delta_t)))).exp();

                let v:f64=prev_v*(self.r*delta_t)+D;
                logger.log(format!("Black|get_output_values (martingale) -> [s:{}|p:{}|d:{}]: prev_v:{}|next_X:{}|sigma:{}|delta_t:{}|b_wgt:{}|v:{}",scenario,start_pos,date,prev_v,next_X,prev_sigma,delta_t,b_wgt,v),"model");
                return Ok(vec![v]);
            }
            else
            {
                //Direct interpolation
                let res=cube.get_item_interp(scenario,start_pos,date,true);
                let (lowerBound,upperBound,r):(usize,usize,f64)=match res {
                    Ok(r)     =>   r,
                    Err(e)    =>   { return Err(format!("Black - {}{}","Error: ",&e)) },
                };
                logger.log(format!("Black|get_output_values (direct) -> [s:{}|p:{}|d:{}|bounds:{}->{}]: {}",scenario,start_pos,date,lowerBound,upperBound,r),"model");
                return Ok(vec![r]);
            }
        }
    }
    // fn get_output_values(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64) -> Result<Vec<f64>,String>
    // {
    //     //TODO: Martingale interpolation
    //     let res=cube.get_item_interp(scenario,start_pos,date,true);
    //     let r:f64=match res {
    //         Ok(r)     =>   r.2,
    //         Err(e)    =>   { return Err(format!("Hw1f - {}{}","Error: ",&e)) },
    //     };
    //     return Ok(vec![r]);
    // }
    fn get_value(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64, _term:f64,logger:&Logger) -> Result<f64,String>
    {
        let res=self.get_output_values(start_pos,cube,raw_start_pos,raw_cube,scenario,date,&logger);
        let v:f64=match res {
            Ok(v)     =>   v[0],
            Err(e)    =>   { return Err(format!("Black - {}{}","Error: ",&e)) },
        };
        return Ok(v);
    }
}