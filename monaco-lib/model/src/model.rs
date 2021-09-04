use data_cube::data_cube::Cube;
use std::any::Any;
use logger::Logger;

pub trait Model
{
    fn init(&mut self) -> ();

    fn get_number_of_variables(&self) -> usize;
    fn get_number_of_outputs(&self) -> usize;
    fn populate_factors(&self,start_raw: usize, raw_factors:&Cube, start:usize, factors:&mut Cube,logger:&Logger) -> ();
    
    fn get_name(&self) -> String;
    fn get_type(&self) -> &str;

    fn get_output_values(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64,logger:&Logger) -> Result<Vec<f64>,String>;
    fn get_value(&self,start_pos:usize, cube:&Cube, raw_start_pos:usize, raw_cube:&Cube, scenario:usize, date:f64, term:f64,logger:&Logger) -> Result<f64,String>;

    fn as_any(&self) -> &dyn Any;
}