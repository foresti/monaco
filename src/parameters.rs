use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RunParameters
{
    pub log_tags:Vec<String>,
    pub num_paths:usize,
    pub time_steps:Vec<f64>,
    pub output_file_variables:String,
    pub output_file_outputs:String,
    pub output_file_exposures:String,
    // pub output_file_exposures_instruments:String,
    pub dump_models:bool,
    pub model_output_dir:String,
    pub dump_model_values:bool,
    pub model_values_terms:Vec<f64>,
    pub output_file_model_values:String,
    pub output_file_cashflows:String,
    pub exercise_output_dir:String,
    pub recycle_randomness:bool,
    pub randomness_file:String
}