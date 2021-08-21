use data_cube::data_cube::Cube;
use model::model::Model;
use model::live_model::LiveModel;
use std::collections::HashMap;
use logger::Logger;

use instrument::instrument::Instrument;

pub fn create_raw_cube(models:&Vec<Box<dyn Model>>,dates:Vec<f64>,num_paths:usize, correlation_matrix:&Vec<f64>) -> Cube
{
    let mut num_of_variables:usize=0;
    for i in 0..models.len()
    {
        num_of_variables+=models[i].get_number_of_variables();
    }

    let normal_variates=math::math::simulate_normal_variates(num_of_variables, num_paths*dates.len(), &correlation_matrix);

    let mut ret_cube=Cube::make_cube(normal_variates,dates.clone(),num_paths,num_of_variables);

    num_of_variables=0;
    for i in 0..models.len()
    {
        for s in num_of_variables..num_of_variables+models[i].get_number_of_variables()
        {
            ret_cube.set_time_series_name(s,&format!("{} [{}]",models[i].get_name(),s-num_of_variables));
        }
        num_of_variables+=models[i].get_number_of_variables();
    }

    return ret_cube;
}

pub fn create_data_cube_from_raw(models:&Vec<Box<dyn Model>>,raw_factors:&Cube, logger:&Logger) -> Cube
{
    let mut num_of_outputs:usize=0;
    let mut num_of_variables:usize=0;

    logger.log("Data cube from raw - Tallying variables and outputs...","controller");
    for i in 0..models.len()
    {
        num_of_outputs+=models[i].get_number_of_outputs();
        num_of_variables+=models[i].get_number_of_variables();
    }

    logger.log("Data cube from raw - Making empty cube...","controller");
    let mut ret_cube=Cube::make_empty_cube(raw_factors.dates.clone(),raw_factors.num_scenarios,num_of_outputs);

    logger.log(format!("Data cube from raw - Results cube - Num paths    : {0}",ret_cube.num_scenarios).as_str(),"controller");
    logger.log(format!("Data cube from raw - Results cube - Num dates    : {0}",ret_cube.dates.len()).as_str(),"controller");
    logger.log(format!("Data cube from raw - Results cube - Numer series : {0}",ret_cube.num_series).as_str(),"controller");

    num_of_outputs=0;
    num_of_variables=0;
    logger.log("Data cube from raw - Populating factors...","controller"); 
    for i in 0..models.len()
    {
        for s in num_of_outputs..num_of_outputs+models[i].get_number_of_outputs()
        {
            ret_cube.set_time_series_name(s,&format!("{} [{}]",models[i].get_name(),s-num_of_outputs));
        }

        let model_name=models[i].get_name();
        logger.log(format!("Data cube from raw - Populating cube for model: {0}",model_name).as_str(),"controller");
        
        logger.log(format!("Data cube from raw - raw_cube: {0}",raw_factors.get_len()).as_str(),"controller");
        logger.log(format!("Data cube from raw - ret_cube: {0}",ret_cube.get_len()).as_str(),"controller");

        models[i].populate_factors(num_of_variables, &raw_factors, num_of_outputs, &mut ret_cube);
        num_of_outputs+=models[i].get_number_of_outputs();
        num_of_variables+=models[i].get_number_of_variables();
    }

    return ret_cube;
}

pub fn compute_paths(models:&Vec<Box<dyn Model>>,time_steps:&Vec<f64>,num_paths:usize, correlation_matrix:&Vec<f64>, logger:&Logger) -> (Cube,Cube)
{
    logger.log("compute_paths - Start","controller");
    //Compute random values for models' variables
    logger.log("compute_exposures - Creating raw cube...","controller");
    let raw_cube=create_raw_cube(&models, time_steps.clone(), num_paths, &correlation_matrix);

    logger.log(format!("Raw cube - Num paths    : {0}",raw_cube.num_scenarios).as_str(),"controller");
    logger.log(format!("Raw cube - Num dates    : {0}",raw_cube.dates.len()).as_str(),"controller");
    logger.log(format!("Raw cube - Numer series : {0}",raw_cube.num_series).as_str(),"controller");

    //Generate factor paths from random sample
    logger.log("compute_exposures - Creating model cube...","controller");
    let paths:Cube=create_data_cube_from_raw(&models, &raw_cube,&logger);

    logger.log(format!("Factor cube - Num paths    : {0}",paths.num_scenarios).as_str(),"controller");
    logger.log(format!("Factor cube - Num dates    : {0}",paths.dates.len()).as_str(),"controller");
    logger.log(format!("Factor cube - Numer series : {0}",paths.num_series).as_str(),"controller");

    return (paths,raw_cube);
}

pub fn create_live_models<'a>(models:&'a Vec<Box<dyn Model>>,paths:&'a Cube,logger:&Logger) -> HashMap<String,LiveModel<'a>>
{
    logger.log("Creating live models...","controller");
    let mut live_models:HashMap<String,LiveModel>=HashMap::new();
    let mut start:usize=0;
    for i in 0..models.len()
    {
        let live_model=LiveModel
        {
            //name: models[i].get_name(),
            cube: &paths,
            start: start,
            model: &models[i]
        };
        live_models.insert(models[i].get_name(),live_model);
        start+=models[i].get_number_of_outputs();
    }
    
    return live_models;
}

// pub fn populate_live_models<'a>(live_models:&'a Vec<LiveModel<'a>>,instruments:&mut Vec<Box<dyn Instrument>>,log:fn(&str)->()) -> ()
// {
//     log("compute_exposures - Populating live models...");
//     for i in 0..instruments.len()
//     {
//         let required_models=instruments[i].get_required_models();
//         for m in 0..required_models.len()
//         {
//             for k in 0..live_models.len()
//             {
//                 if live_models[k].name==required_models[m]
//                 {
//                     instruments[i].set_live_model(&live_models[k]);
//                     break;
//                 }
//             }
//         }
//     }
// }

/// Top-level function
/// 
/// #Arguments
/// 
/// * `models` - Models used for factor evolution
/// * `instruments` - Instruments to price on the simulated paths
/// * `time_steps` - List of time steps (years fractions) to be used for the simulation
/// * `num_paths` - Number of paths to simulate
/// * `correlation_matrix` - Correlation matrix between the models' variables
/// 
/// # Remarks
/// 
/// The order of correlations between the factors is the same of the variables in the `models` paramters
pub fn compute_exposures(instruments:&Vec<Box<dyn Instrument>>,mut results_cube:Cube,live_models:&HashMap<String,LiveModel>, logger:&Logger) -> (Cube,Vec<Vec<Vec<(f64,f64)>>>,Vec<Cube>)
{ 
    // log("compute_exposures - Making empty results cube...");
    // let results_cube=Cube::make_empty_cube(time_steps.clone(),num_paths,instruments.len());

    // log(format!("Results cube - Num paths    : {0}",results_cube.num_scenarios).as_str());
    // log(format!("Results cube - Num dates    : {0}",results_cube.dates.len()).as_str());
    // log(format!("Results cube - Num series : {0}",results_cube.num_series).as_str());

    let mut cashflows:Vec<Vec<Vec<(f64,f64)>>>=Vec::new();
    let mut exercise_cubes:Vec<Cube>=Vec::new();

    //Price instruments on paths
    logger.log("compute_exposures - Generating exposures...","controller");
    for i in 0..instruments.len()
    {
        logger.log(&format!("compute_exposures - Computing values for instrument: {} ({}/{})...",instruments[i].get_name(),i,instruments.len())[..],"controller");
        let (instrument_cashflows,instrument_exercise_cube)=instruments[i].compute_values(i,&mut results_cube,&live_models,logger);
        results_cube.set_time_series_name(i,&instruments[i].get_name());

        cashflows.push(instrument_cashflows);
        exercise_cubes.push(instrument_exercise_cube);
    }

    return (results_cube,cashflows,exercise_cubes);
}