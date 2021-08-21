use macros::debug;
use data_cube::data_cube::Cube;
use crate::model::Model;

//Utility function
pub fn get_fwd_rate(model:&LiveModel,scenario:usize,date:f64,start:f64,end:f64) -> f64
{
    let short=model.get_value(scenario, date, start).unwrap();
    let long=model.get_value(scenario, date, end).unwrap();

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

    pub model:&'a Box<dyn Model>
}

impl LiveModel<'_>
{
    pub fn get_variable_values(&self,scenario:usize, date:f64) -> Vec<f64>
    {
        let num_var:usize=self.model.get_number_of_variables();
        //debug!(format!("LiveModel|get_variable_values -> name: {}, scenario: {}, date: {}, num_var: {}, self.start: {}",self.model.get_name(),scenario,date,num_var,self.start));
        let mut values:Vec<f64>=vec![0.0;num_var];
        for i in 0..num_var
        {
            values[i]=self.cube.get_item_interp(scenario,self.start+i, date,true).unwrap().2;
        }
        return values;
    }
    pub fn get_value(&self, scenario:usize, date:f64, term:f64) -> Result<f64,String>
    {
        return self.model.get_value(self.start, &self.cube, scenario, date, term);
    }
}