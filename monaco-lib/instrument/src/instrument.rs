use data_cube::data_cube::Cube;
use model::live_model::LiveModel;
use std::collections::HashMap;
use logger::Logger;
//use std::any::Any;

pub trait Instrument
{
    fn get_name(&self) -> String;
    fn compute_values(&self,start:usize,result_cube:&mut Cube,live_models:&HashMap<String,LiveModel>,logger:&Logger) -> (Vec<Vec<(f64,f64)>>,Cube);

    //fn as_any(&self) -> &dyn Any;
}