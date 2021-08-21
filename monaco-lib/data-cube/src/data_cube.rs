/// Data cube implementation
use std::vec;
use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize,Deserialize,Debug)]
/// A data cube is a 3-dimensional structure holding f64 data
/// The three dimensions are scenarios, items (time series), and time (a list of dates)
/// The list of time series names is provided to facilitate usage.
/// The time series names are initially set to the ordinal position of the series converted to string (e.g. "3")
/// The method 'set_time_series_name' can be used to change the time series names.
pub struct Cube
{
    data: Vec<f64>,
    pub dates: Vec<f64>,
    pub time_series_names:Vec<String>,
    pub num_scenarios: usize,
    pub num_series: usize
}

impl Cube
{
    /// Creates an empty data cube
    /// 
    /// # Arguments
    /// 
    /// * `dates` - Vector of dates describing the date dimension
    /// * `num_scenarios` - Scenario dimension length
    /// * `num_series` - Series dimension length
    pub fn make_empty_cube(dates:Vec<f64>, num_scenarios:usize, num_series:usize) -> Cube
    {
        let mut names:Vec<String>=vec!["".to_string();num_series];
        for s in 0..num_series
        {
            names[s]=s.to_string();
        }
        let c=Cube
        {
            data:vec![0.0;dates.len()*num_scenarios*num_series],
            dates:dates,
            num_scenarios:num_scenarios,
            num_series:num_series,
            time_series_names:names
        };
        return c;
    }

    /// Creates a data cube
    /// 
    /// # Arguments
    /// 
    /// * `data` - Vector containing the data to be stored in the cube
    /// * `dates` - Vector of dates describing the date dimension
    /// * `num_scenarios` - Scenario dimension length
    /// * `num_series` - Series dimension length
    /// 
    /// # Remarks 
    /// 
    /// `dates.len()*num_scenarios*num_series` MUST equal `data.len()`
    pub fn make_cube(data:Vec<f64>, dates:Vec<f64>, num_scenarios: usize, num_series:usize) -> Cube
    {
        let mut names:Vec<String>=vec!["".to_string();num_series];
        for s in 0..num_series
        {
            names[s]=s.to_string();
        }
        let cube=Cube
        {
            data: data,
            dates: dates,
            num_scenarios: num_scenarios,
            num_series: num_series,
            time_series_names:names
        };
        cube
    }

    pub fn get_len(&self) -> usize
    {
        return self.data.len();
    }

    /// Returns a date from the dates dimension
    /// 
    /// # Arguments
    /// 
    /// * `index` - Date to be found
    pub fn get_date(&self, index: usize) -> Result<f64,String>
    {
        if index<self.dates.len()
        {
            return Ok(self.dates[index]);
        }
        else
        {
            return Err("Index out of bounds.".to_string());
        }
    }

    /// Gets the index of a particular date in the date dimension
    /// 
    /// # Arguments
    /// 
    /// * `date` - Date to be found
    pub fn get_date_index(&self, date: f64) -> Result<usize,usize>
    {
        return self.dates.binary_search_by(|v| v.partial_cmp(&date).expect("NaN"));
    }

    /// Gets the index of an item in the data vector
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `series` - Series index
    /// * `date` - Date
    pub fn get_index(&self, scenario: usize, series: usize, dt_pos: usize) -> Result<usize,String>
    {
        let pos=self.dates.len()*self.num_series*scenario+dt_pos*self.num_series+series;
        if pos<self.data.len() 
            { return Ok(pos) }
        else
            { return Err("No such position in cube.".to_string()) }
    }

    /// Gets the index of an item in the data vector
    /// If the date does not match any dates in the date dimension, the index of the corresponding point for the largest previous date is returned
    /// The tuple returned is (date index of the actual or previous date, data index)
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `series` - Series index
    /// * `date` - Date
    /// 
    /// # Remarks
    /// 
    /// If the date argument precedes the first date in the date dimension an error is returned 
    pub fn get_index_last(&self, scenario: usize, series: usize, date: f64) -> Result<(usize,usize),String>
    {
        let dt_pos_result=self.get_date_index(date);
        let dt_pos:usize=match dt_pos_result 
        {
            Ok(num)             =>  num,
            Err(num) if num>0   =>  num-1,
            Err(_)              =>  { return Err("Date before earliest date.".to_string()) }
        };
        let pos=self.dates.len()*self.num_series*scenario+dt_pos*self.num_series+series;
        if pos<self.data.len() 
            { return Ok((dt_pos,pos)) }
        else
            { return Err("No such position in cube.".to_string()) }
    }

    /// Sets the value of an item in the data vector
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `series` - Series index
    /// * `date` - Date
    /// * `value` - Value to be set
    pub fn set_item(&mut self, scenario: usize, series: usize, dt_pos: usize, value: f64) -> Result<(),String>
    {
        let pos=self.get_index(scenario,series,dt_pos);

        match pos
        {
            Ok(idx)     => { self.data[idx]=value; return Ok(()) },
            Err(str)    => return Err(str)
        }
    }

    /// Gets an item in the data vector
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `series` - Series index
    /// * `date` - Date
    pub fn get_item(&self, scenario: usize, series: usize, dt_pos: usize) -> Result<f64,String>
    {
        let pos=self.dates.len()*self.num_series*scenario+dt_pos*self.num_series+series;

        if pos<self.data.len() 
            { return Ok(*self.data.get(pos).unwrap()) }
        else
            { return Err("No such position in cube.".to_string()) }
    }

    // /// Gets an item in the data vector
    // /// 
    // /// # Arguments
    // /// 
    // /// * `scenario` - Scenario index
    // /// * `series` - Series index
    // /// * `date` - Date
    // pub fn get_item(&self, scenario: usize, series: usize, date: f64) -> Result<&f64,String>
    // {
    //     let pos=self.get_index(scenario,series,date);

    //     match pos
    //     {
    //         Ok(idx)     => return Ok(self.data.get(idx).unwrap()),
    //         Err(str)    => return Err(str)
    //     }
    // }

    // /// Gets an item in the data vector
    // /// 
    // /// # Arguments
    // /// 
    // /// * `scenario` - Scenario index
    // /// * `series` - Series index
    // /// * `date` - Date
    // /// 
    // /// # Remarks
    // /// 
    // /// This method returns a copy of the value (instead of a reference)
    // pub fn get_item_val(&self, scenario: usize, series: usize, date: f64) -> Result<f64,String>
    // {
    //     let pos=self.get_index(scenario,series,date);

    //     match pos
    //     {
    //         Ok(idx)     => return Ok(*self.data.get(idx).unwrap()),
    //         Err(str)    => return Err(str)
    //     }
    // }

    /// Gets a slice of the data vector corresponding to a scenario
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    pub fn get_scenario(&self,scenario: usize) -> Result<&[f64],String>
    {
        let start=self.dates.len()*self.num_series*scenario;
        let end=start+self.dates.len()*self.num_series;

        if  start<self.data.len() && end<self.data.len()
        {
            return Ok(&self.data[start..end]);
        }
        else
        {
            return Err("Invalid bounds.".to_string());
        }
    }


    /// Gets a slice of the data vector corresponding to a price vector for a scenario
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `date_idx` - Index of the date in the date dimension
    pub fn get_item_vector(&self,scenario: usize, dt_pos: usize) -> Result<&[f64],String>
    {
        let start=self.dates.len()*self.num_series*scenario+self.num_series*dt_pos;
        let end=start+self.num_series;

        return Ok(&self.data[start..end]);
    }

    /// Gets a slice of the data vector corresponding to a price vector for a scenario
    /// The tuple returned is (date index, item vector)
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `date` - Date of the vector
    /// 
    /// # Remarks
    /// 
    /// If the date is not found, the vector as of the largest previous date is returned 
    pub fn get_item_vector_last(&self,scenario: usize,date: f64) -> Result<(usize,&[f64]),String>
    {
        let dt_pos=self.get_date_index(date);
        match dt_pos
        {
            Ok(p)           =>  {   
                                    let start=self.dates.len()*self.num_series*scenario+self.num_series*p;
                                    let end=start+self.dates.len()*self.num_series;

                                    return Ok((p,&self.data[start..end]));
                                },
            Err(p) if p>0  =>   {   
                                
                                    let start=self.dates.len()*self.num_series*scenario+self.num_series*(p-1);
                                    let end=start+self.dates.len()*self.num_series;

                                    return Ok((p-1,&self.data[start..end]));
                                },
            Err(_)          =>  { return Err("Date before earliest date.".to_string()) }
        }
    }

    /// Gets an item in the data vector
    /// The tuple returned is (date index, item)
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `series` - Series index
    /// * `date` - Date
    /// 
    /// # Remarks
    /// 
    /// If the date is not found, data for the largest previous date is returned
    pub fn get_item_last(&self, scenario: usize, series: usize, date: f64) -> Result<(usize,f64),String>
    {
        let pos=self.get_index_last(scenario,series,date);

        match pos
        {
            Ok((p,idx))    => return Ok((p,*self.data.get(idx).unwrap())),
            Err(str)        => return Err(str)
        }
    }

    /// Gets a slice of the data vector corresponding to an item vector for a scenario
    /// The tuple returned is (previous date index,following date index,item vector)
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `date` - Date of the vector
    /// 
    /// # Remarks
    /// 
    /// If the date is not found, an interpolated value is returned.
    /// If the date follows the last date in the date dimension, the last date vector is used.
    /// If the date precedes the first date in the date dimension, an error is returned.
    pub fn get_item_vector_interp(&self,scenario: usize,date: f64, allow_out_of_bounds:bool) -> Result<(usize,usize,Vec<f64>),String>
    {
        let dt_pos_result=self.get_date_index(date);

        let down:usize;
        let up:usize;
        match dt_pos_result
        {
            Ok(p)   =>  {up=p; down=p;},
            Err(p)  =>  {up=p; down=if p==0 {0} else{p-1}}    
        }

        if up==down
        {
            let slice=self.get_item_vector(scenario,up).unwrap();
            let mut v=vec![0.0; slice.len()];
            v.copy_from_slice(slice);
            return Ok((down,up,v));
        }
        else
        {
            if up==0
            {
                if allow_out_of_bounds
                {
                    let slice=self.get_item_vector(scenario,0).unwrap();
                    let mut v=vec![0.0; slice.len()];
                    v.copy_from_slice(slice);
                    return Ok((down,up,v));
                }
                else
                {
                    return Err("Date before earliest date.".to_string());
                }
            }
            else
            {
                if up==self.dates.len()
                {
                    if allow_out_of_bounds
                    {
                        let slice=self.get_item_vector(scenario,self.dates.len()-1).unwrap();
                        let mut v=vec![0.0; slice.len()];
                        v.copy_from_slice(slice);
                        return Ok((down,up,v));
                    }
                    else
                    {
                        return Err("Date after last date.".to_string());
                    }
                }
                else
                {
                    let up_slice=self.get_item_vector(scenario, up).unwrap();
                    let down_slice=self.get_item_vector(scenario, down).unwrap();
                    let period=self.dates[up]-self.dates[down];
                    let diff=date-self.dates[down];

                    let mut v=vec::Vec::new();
                    for i in 0..self.num_series
                    {
                        let res=((up_slice[i]-down_slice[i])/period)*diff+down_slice[i];
                        v.push(res);
                    }

                    return Ok((down,up,v));
                }
            }
        }
    }

    /// Gets an item in the data vector
    /// The tuple returned is (previous date index,following date index,item)
    /// 
    /// # Arguments
    /// 
    /// * `scenario` - Scenario index
    /// * `series` - Series index
    /// * `date` - Date
    /// 
    /// # Remarks
    /// 
    /// If the date is not found, an interpolated value is returned.
    /// If the date follows the last date in the date dimension, the last date vector is used.
    /// If the date precedes the first date in the date dimension, an error is returned.
    pub fn get_item_interp(&self, scenario: usize, series: usize, date: f64, allow_out_of_bounds:bool) -> Result<(usize,usize,f64),String>
    {
        let dt_pos_result=self.get_date_index(date);

        let down:usize;
        let up:usize;
        match dt_pos_result
        {
            Ok(p)   =>  {up=p; down=p;},
            Err(p)  =>  {up=p; down= if p==0 {0} else {p-1}}    
        }

        if up==down
        {
            // println!("Get item interp - scenario: {}|series: {}|date: {}|pos: {}|value:{}",scenario,series,date,pos,*self.data.get(pos).unwrap());
            return Ok((down,up,self.get_item(scenario,series,up).unwrap()));
        }
        else
        {
            if up==self.dates.len()
            {
                if allow_out_of_bounds
                {
                    //let pos=(up-1)*self.num_series*scenario+series;
                    // println!("Get item interp - scenario: {}|series: {}|date: {}|pos: {}|value:{}",scenario,series,date,pos,*self.data.get(pos).unwrap());
                    return Ok((down,up,self.get_item(scenario,series,self.dates.len()-1).unwrap()));
                }
                else
                {
                    return Err("Date after last date.".to_string());
                }
            }
            else
            {
                if up==0
                {
                    if allow_out_of_bounds
                    {
                        //let pos=series;
                        // println!("Get item interp - scenario: {}|series: {}|date: {}|pos: {}|value:{}",scenario,series,date,pos,*self.data.get(pos).unwrap());
                        return Ok((down,up,self.get_item(scenario,series,up).unwrap()));
                    }
                    else
                    {
                        return Err("Date before earliest date.".to_string());
                    }
                }
                else
                {
                    //let pos_up=self.dates.len()*self.num_series*scenario+up*self.num_series+series;
                    //let pos_down=self.dates.len()*self.num_series*scenario+down*self.num_series+series;

                    //let period=self.dates[up].signed_duration_since(self.dates[down]).num_days() as f64;
                    //let diff=date.signed_duration_since(self.dates[down]).num_days() as f64;

                    let period=self.dates[up]-self.dates[down];
                    let diff=date-self.dates[down];

                    //let up_value=self.data.get(pos_up).unwrap();
                    //let down_value=self.data.get(pos_down).unwrap();

                    let up_value=self.get_item(scenario, series,up).unwrap();
                    let down_value=self.get_item(scenario,series,down).unwrap();

                    let res=((up_value-down_value)/period)*diff+down_value;

                    // println!("Get item interp (*) - scenario:{}|series:{}|date:{}|down:{}|up: {}|value:{}",scenario,series,date,down,up,res);
                    return Ok((down,up,res));
                }
            }
        }
    }

    /// Sets the name for one of the time series
    /// 
    /// # Arguments
    /// 
    /// * `series_idx` - Series index
    /// * `name` - Series name
    pub fn set_time_series_name(&mut self, series_idx: usize, name:&str) -> ()
    {
        self.time_series_names[series_idx]=name.to_string();
    }
}
