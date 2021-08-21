use std::any::Any;
use crate::model::Model;
use data_cube::data_cube::Cube;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fixed
{
    pub name: String,
    pub value: f64
}

impl Model for Fixed
{
    fn as_any(&self) -> &dyn Any { self }

    fn init(&mut self) -> () { }
    fn get_name(&self) -> String { return self.name.clone(); }
    fn get_type(&self) -> &str { return "fixed"; }
    fn get_number_of_variables(&self) -> usize{return 0;}
    fn get_number_of_outputs(&self) -> usize {return 0;}

    fn populate_factors(&self,_start_raw: usize,_raw_factors:&Cube,_start:usize,_factors:&mut Cube) -> () {}
    fn get_value(&self,_start_pos:usize,_cube:&Cube,_scenario:usize,_date:f64, _term:f64) -> Result<f64,String>
    {
        return Ok(self.value);
    }
}