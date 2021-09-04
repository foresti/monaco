use std::any::Any;
use crate::model::Model;
use data_cube::data_cube::Cube;
use curve::curve::Curve;
use curve::curve::IrCurve;
use serde::{Serialize, Deserialize};
use macros::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hw1f
{
    pub name: String,
    pub term_structure: Vec<(f64,f64)>,
    pub thetas: Vec<(f64,f64)>,
    pub a: Vec<(f64,f64)>,
    pub sigmas: Vec<(f64,f64)>,
    pub initial_rate: f64
}

impl Hw1f
{
    const DELTA_T:f64=0.0001;
}

impl Hw1f
{    
    pub fn thetas_from_term_structure(&self) -> Vec<(f64,f64)>
    {
        let mut thetas:Vec<(f64,f64)>=vec![(0.0,0.0);self.term_structure.len()];

        let crv=Curve
        {
            data: self.term_structure.clone()
        };

        for i in 0..self.term_structure.len()
        {
            let t:f64=self.term_structure[i].0;
            let a:f64=math::math::interpolate(&self.a, t);
            let sigma:f64=math::math::interpolate(&self.sigmas, t);

            let fwd=crv.get_cont_fwd(t-Hw1f::DELTA_T,t+Hw1f::DELTA_T).unwrap();
            let x=1.0-(-a*t).exp();
            let adj:f64=((sigma*sigma)/(2.0*a*a))*(x*x);
            let theta=fwd+adj;
            thetas[i]=(t,theta);
        }

        return thetas;
    }

    fn inst_forward(&self, t:f64) -> f64
    {
        let t1:f64 = t - Hw1f::DELTA_T / 2.0;
        let t2:f64 = t + Hw1f::DELTA_T / 2.0;

        let p1:f64 = (-math::math::interpolate(&self.term_structure,t1)*t1).exp();
        let p2:f64 = (-math::math::interpolate(&self.term_structure,t2)*t2).exp();

        //debug!(format!("hw1f|InstFwd -> name: {}, interp_t1: {}, interp_t2: {}, t1: {}, t2: {}, p1: {}, p2: {}",self.name,math::math::interpolate(&self.term_structure,t),math::math::interpolate(&self.term_structure,t2),t1,t2,p1,p2));

        let f:f64 = p2 / p1;

        return f.ln()/(t2-t1);
    }
    #[allow(non_snake_case)]
    fn A(&self, t:f64, T:f64) -> f64
    {
        let pt:f64=(-math::math::interpolate(&self.term_structure,t)*t).exp();
        let pT:f64=(-math::math::interpolate(&self.term_structure,T)*T).exp();

        let instFwd:f64 = self.inst_forward(t);

        let sigma:f64=math::math::interpolate(&self.sigmas, t);
        let a:f64=math::math::interpolate(&self.a, t);
        let B:f64=self.B(t, T);
        let v1:f64=-B * instFwd;
        let e:f64=(-a*T).exp()-(-a*t).exp();
        let f:f64=(2.0*a*t).exp()-1.0;
        let v2:f64=(sigma*sigma*e*e*f)/(4.0*a*a*a);
        // let temp:f64 = math::math::interpolate(&self.sigmas, t) * self.B(t, T);
        // let value:f64 = -self.B(t, T) * instFwd - 0.25 * temp * temp * self.B(0.0, 2.0 * t);
        
        let value:f64 =  v1-v2;
        let a_val=value.exp() * (pT / pt);
        //debug!(format!("hw1f|A -> name: {}, B: {}, instFwd: {}, sigma: {}, a: {}, v1:{}, e: {}, f: {}, v2: {}, pt: {}, pT: {}, a_val: {}",self.name,B,instFwd,sigma,a,v1,e,f,v2,pt,pT,a_val));
        return a_val;
    }
    #[allow(non_snake_case)]
    fn B(&self, t:f64, T:f64) -> f64
    {
        let a=math::math::interpolate(&self.a, t);
        let b=1.0 - f64::exp(-a * (T - t));
        //debug!(format!("hw1f|B ->  name:{}, b/a: {}, a: {}, b: {}",self.name,b/a,a,b));
        return b/a;
    }
}

impl Model for Hw1f
{
    fn as_any(&self) -> &dyn Any { self }

    fn init(&mut self) -> ()
    {
        let thetas=Hw1f::thetas_from_term_structure(&self);
        self.thetas=thetas;
        //for i in 0..self.thetas.len()
        //{
        //    self.thetas[i].1=thetas[i].1;
        //}
    }
    fn get_name(&self) -> String
    {
        return self.name.clone();
    }
    fn get_type(&self) -> &str
    {
        return "hw1f";
    }
    fn get_number_of_variables(&self) -> usize
    {
        return 1;
    }
    fn get_number_of_outputs(&self) -> usize
    {
        return 1;
    }

    fn populate_factors(&self,start_raw: usize, raw_factors:&Cube, start:usize, factors:&mut Cube) -> ()
    {
        for s in 0..factors.num_scenarios
        {
            for dt_idx in 0..factors.dates.len()
            {
                let t:f64=factors.dates[dt_idx];
                if t==0.0
                {
                    factors.set_item(s, start, 0, self.initial_rate).unwrap();
                }
                else
                {
                    let t:f64=factors.dates[dt_idx];
                    let prev_t:f64= if dt_idx==0 {0.0} else {factors.dates[dt_idx - 1]};  
                    let delta_t:f64 = t - prev_t;
                    let phi_t:f64=math::math::interpolate(&self.thetas, t);
                    
                    let prev_r:f64=if dt_idx==0 { self.initial_rate } else { factors.get_item(s, start, dt_idx-1).unwrap() };

                    let r1:f64=phi_t*delta_t;
                    let r2:f64=-math::math::interpolate(&self.a, t)*prev_r*delta_t;
                    let r3:f64=math::math::interpolate(&self.sigmas, t)*f64::sqrt(delta_t)*raw_factors.get_item(s, start_raw, dt_idx).unwrap();
                    let r=prev_r+r1+r2+r3;
                    //debug!(format!("hw1f|Populate factors -> name: {0}, s: {1}, dt_idx: {2}, delat_t: {3}, phi_t: {4}, r1: {5}, r2: {6}, r3: {7}, r: {8}, prev_r: {9}",self.name,s,dt_idx,delta_t,phi_t,r1,r2,r3,r,prev_r));
                    factors.set_item(s, start, dt_idx, r).unwrap();
                }
            }
        }
    }
    #[allow(non_snake_case)]
    fn get_output_values(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64) -> Result<Vec<f64>,String>
    {
        //Martingale interpolation
        let prev_r_res=cube.get_item_last(scenario,start_pos,date);
        let (prev_dt_idx,prev_r)=match prev_r_res
        {
            Ok((p,q)) => (p,q),
            Err(e) => return Err(format!("Hw1f - {}{}","Error: ",&e)),
        };

        let prev_dt_res=cube.get_date(prev_dt_idx);
        let prev_dt=match prev_dt_res
        {
            Ok(v) => v,
            Err(e) => return Err(format!("Hw1f - {}{}","Error: ",&e)),
        };

        let next_dt_idx=prev_dt_idx+1;
        let next_dt_res=cube.get_date(next_dt_idx);
        let next_dt=match next_dt_res
        {
            Ok(v) => v,
            Err(e) => return Err(format!("Hw1f - {}{}","Error: ",&e)),
        };

        let delta_t=date-prev_dt;
        let phi_t:f64=math::math::interpolate(&self.thetas, date);
        let a=-math::math::interpolate(&self.a, date)*prev_r*delta_t;

        let r1=phi_t*delta_t;
        let r2=a*prev_r*delta_t;

        let (prev_X_pos,_prev_X)=match raw_cube.get_item_last(scenario,raw_start_pos,date)
        {
            Ok((p,q)) => (p,q),
            Err(e) => return Err(format!("Hw1f - {}{}","Error: ",&e)),
        };
        let next_X=match raw_cube.get_item(scenario,raw_start_pos,prev_X_pos+1)
        {
            Ok(v) => v,
            Err(e) =>  return Err(format!("Hw1f - {}{}","Error: ",&e)),
        };
        
        let prev_sigma=math::math::interpolate(&self.sigmas, prev_dt);
        let b_wgt:f64=((date-prev_dt)/(next_dt-prev_dt)).sqrt();
        //Sigma is defined at the beginning of each time step
        let r3=prev_sigma*(date-prev_dt)*((b_wgt*next_X)-0.5*(prev_sigma*prev_sigma*(date-prev_dt)));

        let r=prev_r+r1+r2+r3;

        //logger.log(format!("Hw1f|get_output_values (martingale) -> [s:{}|p:{}|d:{}] prev_v:{}|next_X:{}|sigma:{}|delta_t:{}|b_wgt:{}|v:{}",scenario,start_pos,date,prev_r,next_X,prev_sigma,delta_t,b_wgt,r),"model");
        return Ok(vec![r]);

        //OLD: Direct interpolation
        // let res=cube.get_item_interp(scenario,start_pos,date,true);
        // let r:f64=match res {
        //     Ok(r)     =>   r.2,
        //     Err(e)    =>   { return Err(format!("Hw1f - {}{}","Error: ",&e)) },
        // };
        //logger.log(format!("Hw1f|get_output_values (direct) -> [s:{}|p:{}|d:{}]: {}",scenario,start_pos,date,r),"model");
        // //return Ok(vec![r]);
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
    fn get_value(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64, term:f64) -> Result<f64,String>
    {
        let res=self.get_output_values(start_pos,cube,raw_start_pos,raw_cube,scenario,date);
        let r:f64=match res {
            Ok(r)     =>   r[0],
            Err(e)    =>   { return Err(format!("Hw1f - {}{}","Error: ",&e)) },
        };
        let v1=self.A(date,date+term);
        let v2=-r*self.B(date,date+term);
        let p=v1*v2.exp();
        //debug!(format!("h1wf|get_value -> name: {}, r: {}, v1: {}, v2: {}, v2exp: {}, p: {}, -p.ln: {}, date: {}, term:{}",self.name,r,v1,v2,v2.exp(),p,-p.ln(),date,term));
        let r=-p.ln()/term;
        return Ok(r);
    }
}