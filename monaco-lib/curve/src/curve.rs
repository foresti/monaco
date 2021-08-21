/// Curve implementation

/// Interest rate curve methods
pub trait IrCurve 
{
    fn get_annu_df(&self,tenor: f64) -> Result<f64, String>;
    fn get_cont_df(&self,tenor: f64) -> Result<f64, String>;
    fn get_annu_fwd(&self,short_tenor: f64,long_tenor: f64) -> Result<f64, String>;
    fn get_cont_fwd(&self,short_tenor: f64,long_tenor: f64) -> Result<f64, String>;
}

#[derive(Clone)]
/// A curve is a collection of tenors
/// Traits are used to determine the correct behaviour
/// All traits use data cubes as the source of data
pub struct Curve
{
    pub data: Vec<(f64,f64)>
}

impl Curve 
{
    /// Gets a value from the curve
    /// 
    /// # Arguments
    /// 
    /// * `tenor' - Tenor for which to retrieve the value
    pub fn get_value(&self,tenor:f64) -> Result<f64, String> 
    {
        return Ok(math::math::interpolate(&self.data,tenor));
    }

    /// Creates a curve from a vector of (term,value) points
    /// 
    /// # Arguments
    /// 
    /// * `data` - vector of (term,value) points
    /// 
    /// # Remarks
    /// 
    /// Terms are  year fractions
    pub fn from_vector(data:&Vec<(f64,f64)>) -> Curve
    {
        let mut d:Vec<(f64,f64)>=Vec::new();
        for i in 0..data.len()
        {
            d.push((data[i].0,data[i].1));
        }
        let c:Curve=Curve
        {
            data:d
        };

        return c;
    }
}

impl IrCurve for Curve
{
    /// Gets a discount factor from the curve (annual compounding)
    /// 
    /// # Arguments
    /// 
    /// * `tenor' - Tenor for which to retrieve the value
    fn get_annu_df(&self,tenor: f64) -> Result<f64, String>
    {
        let rate=self.get_value(tenor);

        match rate
        {
            Ok(r)   =>  return Ok((1.0+r).powf(-tenor)),
            Err(s)  =>  return Err(s)
        }
    }
    /// Gets a discount factor from the curve (continuous compounding)
    /// 
    /// # Arguments
    /// 
    /// * `tenor' - Tenor for which to retrieve the value
    fn get_cont_df(&self,tenor: f64) -> Result<f64, String>
    {
        let rate=self.get_value(tenor);
        match rate
        {
            Ok(r)   =>  return  Ok((r*(-tenor)).exp()),
            Err(s)  =>  return Err(s)
        }
    }
    /// Gets a forward rate from the curve (annual compounding)
    /// 
    /// # Arguments
    /// 
    /// * `short_tenor' - Tenor to the forward period start
    /// * `long_tenor' - Tenor to the forward period end
    fn get_annu_fwd(&self,short_tenor: f64,long_tenor: f64) -> Result<f64, String>
    {
        let short_df=self.get_annu_df(short_tenor);
        let long_df=self.get_annu_df(long_tenor);

        match (short_df,long_df)
        {
            (Ok(s),Ok(l))   =>  {
                                    let fwd_df=l/s;
                                    let res=fwd_df.powf(-1.0/(long_tenor-short_tenor))-1.0;
                                    return Ok(res);
                                },

            _               =>  Err("Error determining discount factors.".to_string())
        }
    }
    /// Gets a forward rate from the curve (continuous compounding)
    /// 
    /// # Arguments
    /// 
    /// * `short_tenor' - Tenor to the forward period start
    /// * `long_tenor' - Tenor to the forward period end
    fn get_cont_fwd(&self,short_tenor: f64,long_tenor: f64) -> Result<f64, String>
    {
        let short_df=self.get_cont_df(short_tenor);
        let long_df=self.get_cont_df(long_tenor);

        match (short_df,long_df)
        {
            (Ok(s),Ok(l))   =>  {
                                    let fwd_df=l/s;
                                    let res=-fwd_df.ln()/(long_tenor-short_tenor);
                                    return Ok(res);
                                },

            _               =>  Err("Error determining discount factors.".to_string())
        }
    }
}
