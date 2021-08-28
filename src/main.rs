mod parameters;

use std::any::Any;
use std::{env, fs};
use std::ffi::OsStr;
use serde::{Serialize, Deserialize};

use parameters::RunParameters;

use monaco_lib::logger::*;
use monaco_lib::data_cube::data_cube::Cube;
use monaco_lib::model::hw1f::Hw1f;
use monaco_lib::model::black::Black;
use monaco_lib::model::fixed::Fixed;
use monaco_lib::model::model::Model;
use monaco_lib::instrument::instrument::Instrument;
use monaco_lib::instrument::vanilla_swap::VanillaSwap;
use monaco_lib::instrument::callable_swap::CallableSwap;
use monaco_lib::exposure_simulation::controller;

fn read_instruments(dir_name:&str,logger:&Logger) -> Vec<Box<dyn Instrument>>
{
    let mut instruments: Vec<Box<dyn Instrument>> = Vec::new();
    for entry in fs::read_dir(dir_name).unwrap()
    {
        let file_entry=entry.unwrap().path();
        let file_name=file_entry.file_name().unwrap().to_str().unwrap();
        logger.log(format!("{}",file_entry.display()).as_str(),"app");
        let x:Vec<&str> = file_name.split("_").collect();
        let contents = fs::read_to_string(&file_entry).expect("Error reading file");

        if x.len()>1
        {
            match x[0]
            {
                "vanilla-swap"  =>  {
                                        println!("Reading vanilla swap...");
                                        let deserialized:VanillaSwap=serde_json::from_str(&contents).unwrap();
                                        instruments.push(Box::new(deserialized));
                                        println!("Vanilla swap created.");
                                    },
                "callable-swap"  =>  {
                                        let deserialized:CallableSwap=serde_json::from_str(&contents).unwrap();
                                        instruments.push(Box::new(deserialized));
                                    },
                &_      =>      ()
            };
        }                           
    }
    return instruments;
}

const VERSION:&str="0.9";
fn main() {
    let mut logger=Logger { log_tags:vec!["app".to_string()] };
    logger.log(format!("Monaco - v{}",VERSION).as_str(),"app");
    
    //let x:Vec<f64>=vec![0.1,0.2,0.3,4.0];
    //let serialized = serde_json::to_string(&x).unwrap();
    //println!("--- serialized = {}", serialized);
    //let deserialized: Vec<f64> = serde_json::from_str(&serialized).unwrap();
    //println!("--- deserialized = {:?}", deserialized);

    let mut models: Vec<Box<dyn Model>> = Vec::new();
    //let mut instruments: Vec<Box<dyn Instrument>> = Vec::new();
    let mut correlation_matrix:Vec<f64>=Vec::new();
    let mut parameters:RunParameters=RunParameters
    {
        log_tags:Vec::new(),
        num_paths:0,
        time_steps:vec![0.0],
        output_file_variables:String::new(),
        output_file_outputs:String::new(),
        output_file_exposures:String::new(),
        // output_file_exposures_instruments:String::new(),
        dump_models:false,
        model_output_dir:String::new(),
        dump_model_values:false,
        model_values_terms:vec![0.0],
        output_file_model_values:String::new(),
        output_file_cashflows:String::new(),
        exercise_output_dir:String::new()
    };

    let args: Vec<String> = env::args().collect();
    logger.log(format!("Config folder: {}",args[1]),"app");
    for entry in fs::read_dir(&args[1]).unwrap()
    {
        let file_entry=entry.unwrap().path();
        let file_name=file_entry.file_name().unwrap().to_str().unwrap();
        logger.log(format!("{}",file_entry.display()),"app");
        let x:Vec<&str> = file_name.split("_").collect();
        let contents = fs::read_to_string(&file_entry).expect("Error reading file!");
        match file_name
        {
            "control.json"    => { println!("Control"); let deserialized:RunParameters=serde_json::from_str(&contents).unwrap(); parameters=deserialized; },
            "correlations.json"     => { println!("Correlations"); let deserialized:Vec<f64>=serde_json::from_str(&contents).unwrap(); correlation_matrix=deserialized; },
            &_                      => {
                                            if x.len()>1
                                            {
                                                    match x[0]
                                                    {
                                                        //Models
                                                        "hw1f"  =>      {
                                                                            let deserialized:Hw1f=serde_json::from_str(&contents).unwrap();
                                                                            models.push(Box::new(deserialized));
                                                                        },
                                                        "black"  =>     {
                                                                            let deserialized:Black=serde_json::from_str(&contents).unwrap();
                                                                            models.push(Box::new(deserialized));
                                                                        },
                                                        "fixed"  =>     {
                                                                            let deserialized:Fixed=serde_json::from_str(&contents).unwrap();
                                                                            models.push(Box::new(deserialized));
                                                                        },
                                                        &_      =>      ()
                                                };
                                            }
                                        }
        }
    }

    logger=Logger { log_tags:parameters.log_tags };

    logger.log("Sorting models...","app");
    models.sort_by(|a, b| a.get_name().partial_cmp(&b.get_name()).unwrap());

    logger.log("Initializing models...","app");
    for i in 0..models.len()
    {
        logger.log(format!("Initializing : {0}",models[i].get_name()).as_str(),"app");
        models[i].init();
    }

    let (paths,raw_cube)=controller::compute_paths(&models, &parameters.time_steps, parameters.num_paths, &correlation_matrix,&logger);
    let live_models=controller::create_live_models(&models, &paths, &raw_cube, &logger);

    println!("Reading instruments...");
    let mut instruments=read_instruments(&args[1],&logger);

    logger.log(format!("Computing exposures..."),"app");
    logger.log(format!("Num models           : {0}",models.len()),"app");
    logger.log(format!("Num instruments      : {0}",instruments.len()),"app");
    logger.log(format!("Num paths            : {0}",parameters.num_paths),"app");
    logger.log(format!("Num time steps       : {0}",parameters.time_steps.len()),"app");
    logger.log(format!("Correlation matrix   : {0}",correlation_matrix.len()),"app");

    // for i in 0..live_models.len()
    // {
    //     log(format!("Live model: {}-{}-{}",live_models[i].name.as_str(),live_models[i].model.get_name().as_str(),live_models[i].start).as_str());
    // }
    //controller::populate_live_models(&live_models, &mut instruments, log);

    let results_cube=Cube::make_empty_cube(parameters.time_steps.clone(),parameters.num_paths,instruments.len());
    let (exposures,cashflows,exercise_cubes)=controller::compute_exposures(&mut instruments,results_cube,&live_models,&logger);

    println!("Results ready.");


    logger.log(format!("Writing exercise cubes to {}...",parameters.exercise_output_dir),"app");

    for c in 0..exercise_cubes.len()
    {
        let ins_name=instruments[c].get_name();
        let serialized_exercise_cube = serde_json::to_string(&exercise_cubes[c]).unwrap();
        let file_name=format!("{}/{} (exercise).json",parameters.exercise_output_dir.as_str(),ins_name);
        logger.log(format!("Writing exercise cube for: {} ({})",ins_name,file_name),"app");
        let write_res=fs::write(file_name, serialized_exercise_cube);
        match write_res
        {
            Ok(_) =>  logger.log(format!("Exercise cube for: {} written.",ins_name),"app"),
            Err(_)  =>   logger.log(format!("Error writing exercise cube for: {}!",ins_name),"app")
        }
    }

    logger.log("Writing cashflows...","app");
    let serialized_casflows = serde_json::to_string(&cashflows).unwrap();
    let write_res=fs::write(parameters.output_file_cashflows, serialized_casflows);
    match write_res
    {
        Ok(_) =>  logger.log(format!("Cashflows written."),"app"),
        Err(_)  =>   logger.log(format!("Error writing cashflows!"),"app")
    }

    // println!("Results cube - Num paths    : {0}",results_cube.num_scenarios);
    // println!("Results cube - Num dates    : {0}",results_cube.dates.len());
    // println!("Results cube - Num series : {0}",results_cube.num_series);

    logger.log(format!("Variables - Num scenarios: {0}",raw_cube.num_scenarios),"app");
    logger.log(format!("Variables - Num dates    : {0}",raw_cube.dates.len()),"app");
    logger.log(format!("Variables - Num series   : {0}",raw_cube.num_series),"app");

    logger.log(format!("Factors - Num scenarios: {0}",paths.num_scenarios),"app");
    logger.log(format!("Factors - Num dates    : {0}",paths.dates.len()),"app");
    logger.log(format!("Factors - Num series   : {0}",paths.num_series),"app");

    logger.log(format!("Exposures - Num scenarios: {0}",exposures.num_scenarios),"app");
    logger.log(format!("Exposures - Num dates    : {0}",exposures.dates.len()),"app");
    logger.log(format!("Exposures - Num series   : {0}",exposures.num_series),"app");

    let serialized_variables = serde_json::to_string(&raw_cube).unwrap();
    let serialized_outputs = serde_json::to_string(&paths).unwrap();
    let serialized_exposures = serde_json::to_string(&exposures).unwrap();

    logger.log(format!("Writing variables to: {}",parameters.output_file_variables),"app");
    let write_res=fs::write(parameters.output_file_variables, serialized_variables);
    match write_res
    {
        Ok(_) =>  logger.log(format!("Variables written."),"app"),
        Err(_)  =>   logger.log(format!("Error writing variables!"),"app")
    }
    logger.log(format!("Writing outputs to: {}",parameters.output_file_outputs),"app");
    let write_res=fs::write(parameters.output_file_outputs, serialized_outputs);
    match write_res
    {
        Ok(_) =>  logger.log(format!("Outputs written."),"app"),
        Err(_)  =>  logger.log(format!("Error writing outputs!"),"app")
    }
    logger.log(format!("Writing exposures to: {}",parameters.output_file_exposures),"app");
    let write_res=fs::write(parameters.output_file_exposures, serialized_exposures);
    match write_res
    {
        Ok(_) =>  logger.log(format!("Exposures written."),"app"),
        Err(_)  =>  logger.log(format!("Error writing exposures!"),"app")
    }

    // let exp_ins_names:Vec<String>=instruments.iter().map(|i| i.get_name()).collect();
    // let serialized_exposures_ins_names = serde_json::to_string(&exp_ins_names).unwrap();
    // logger.log(format!("Writing exposures instrument names to: {}",parameters.output_file_exposures_instruments),"app");
    // let write_res=fs::write(parameters.output_file_exposures_instruments, serialized_exposures_ins_names);
    // match write_res
    // {
    //     Ok(_) =>  logger.log(format!("Exposures instrument names written."),"app"),
    //     Err(_)  =>  logger.log(format!("Error writing exposures instrument names!"),"app")
    // }

    if parameters.dump_models
    {
        // println!("Writing live models to folder: {}",parameters.model_output_dir);
        // let serialized_live_models = serde_json::to_string(&live_models).unwrap();
        // let _=fs::write(format!("{}{}",parameters.model_output_dir.as_str(),"/live_models.json"),serialized_live_models);

        logger.log(format!("Writing models to folder: {}",parameters.model_output_dir),"app");
        for i in 0..models.len()
        {
            let model_name=models[i].get_type();
            let mut serialized_model=String::new();
            match model_name
            {
                "hw1f"      =>  {
                                    let model=models[i].as_any().downcast_ref::<Hw1f>().unwrap();
                                    serialized_model = serde_json::to_string(model).unwrap();
                                },
                "black"     =>  {
                                    let model=models[i].as_any().downcast_ref::<Black>().unwrap();
                                    serialized_model = serde_json::to_string(model).unwrap();
                                },
                "fixed"     =>  {
                                    let model=models[i].as_any().downcast_ref::<Fixed>().unwrap();
                                    serialized_model = serde_json::to_string(model).unwrap();
                                },
                &_          =>  ()
            }
            let _=fs::write(format!("{}{}{}{}",parameters.model_output_dir.as_str(),"/",models[i].get_name().as_str(),".json"),serialized_model);
        }
    }

    if parameters.dump_model_values
    {
        logger.log(format!("Writing models vaues to: {}",parameters.output_file_model_values),"app");
        let mut model_values=Cube::make_empty_cube(paths.dates.clone(),paths.num_scenarios,live_models.len()*parameters.model_values_terms.len());
        //println!("{}",model_values.dates[0]);
        let mut keys:Vec<String>=live_models.keys().map(|k| k.to_string()).collect();
        keys.sort();

        let mut name_idx=0;
        for key in keys.iter()
        {
            for j in 0..parameters.model_values_terms.len()
            {
                let name:String=format!("{} [{}]",live_models[key].model.get_name(),parameters.model_values_terms[j]);
                model_values.set_time_series_name(name_idx,&name);
                name_idx+=1;
            }
        }
        for s in 0..model_values.num_scenarios
        {
            for dt_idx in 0..paths.dates.len()
            {
                let mut i:usize=0;
                for key in keys.iter()
                {
                    for j in 0..parameters.model_values_terms.len()
                    {
                        //println!("{}-{}-{}",s,dt_idx,paths.dates[dt_idx]);
                        let v:f64=live_models[key].get_value(s, paths.dates[dt_idx], parameters.model_values_terms[j]).unwrap();
                        let f:f64=live_models[key].cube.get_item_interp(s, live_models[key].start, paths.dates[dt_idx], true).unwrap().2;
                        //logger.log(format!("Model value: {} (factor: {}) - model: {} - scenario: {} - series: {} - date index: {} - date: {} - term: {}",v,f,live_models[key].model.get_name(),s,live_models[key].start, dt_idx,paths.dates[dt_idx], parameters.model_values_terms[j]),"app");
                        let _=model_values.set_item(s,i*parameters.model_values_terms.len()+j,dt_idx,v);
                    }
                    i=i+1;
                }
            }
        }
        let serialized_model_values = serde_json::to_string(&model_values).unwrap();
        let write_res=fs::write(parameters.output_file_model_values, serialized_model_values);
        match write_res
        {
            Ok(_) =>  logger.log(format!("Model values written."),"app"),
            Err(_)  =>   logger.log(format!("Error writing model values!"),"app")
        }
    }

    logger.log(format!("Done."),"app");
}