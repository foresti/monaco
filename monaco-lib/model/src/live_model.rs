use macros::debug;
use data_cube::data_cube::Cube;
use crate::model::Model;
use logger::Logger;

//Utility function
pub fn get_fwd_rate(model:&LiveModel,scenario:usize,date:f64,start:f64,end:f64,logger:&Logger) -> f64
{
    let short=model.get_value(scenario, date, start,&logger).unwrap();
    let long=model.get_value(scenario, date, end,&logger).unwrap();

    let df_short=(-short*(start-date)).exp();
    let df_long=(-long*(end-date)).exp();

    let df_fwd=df_long/df_short;

    let fwd_rate=-df_fwd.ln()/(end-start);

    return fwd_rate;
}

pub struct LiveModel<'a>
{
    pub cube:&'a Cube,
    pub start:usize,

    pub raw_cube:&'a Cube,
    pub raw_start:usize,

    pub model:&'a Box<dyn Model>
}

impl LiveModel<'_>
{
    pub fn get_variable_values(&self,scenario:usize, date:f64,logger:&Logger) -> Vec<f64>
    {
        let num_var:usize=self.model.get_number_of_outputs();
        //debug!(format!("LiveModel|get_variable_values -> name: {}, scenario: {}, date: {}, num_var: {}, self.start: {}",self.model.get_name(),scenario,date,num_var,self.start));
        // let mut values:Vec<f64>=vec![0.0;num_var];
        // for i in 0..num_var
        // {
        //     //values[i]=self.cube.get_item_interp(scenario,self.start+i, date,true).unwrap().2;
        //     values[i]=self.model.get_output_value(self.start,&self.cube, self.raw_start, &self.raw_cube, scenario,self);
        // }
        let values=self.model.get_output_values(self.start,&self.cube, self.raw_start, &self.raw_cube, scenario,date,&logger).unwrap();
        return values;
    }
    pub fn get_value(&self, scenario:usize, date:f64, term:f64,logger:&Logger) -> Result<f64,String>
    {
        return self.model.get_value(self.start, &self.cube, self.raw_start, &self.raw_cube, scenario, date, term,&logger);
    }
}