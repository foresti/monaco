use crate::matrix::*;
use crate::math::*;

/// Multi-variate linear regression calculator
pub struct LinearRegressor
{
    pub parameters: Vec<f64>
}

impl LinearRegressor
{
    /// Create regressor from vectors of x: (n x m) and y: (n x 1) values
    ///
    ///  # Arguments
    /// 
    /// * `x` - X values
    /// * `y` - Y values
    pub fn create_regressor(x:&Vec<f64>,y:&Vec<f64>) -> Result<LinearRegressor,String>
    {
        if y.len()>0
        {
            let num_variables=x.len()/y.len();
            let mut m_data:Vec<f64>=vec![0.0;x.len()+y.len()];

            for i in 0..y.len()
            {
                for j in 0..num_variables
                {
                    m_data[i*(num_variables+1)+j]=x[i*num_variables+j];
                }
                m_data[i*(num_variables+1)+num_variables]=1.0;
            }

            let m_x=Matrix
            {
                data: m_data,
                rows: y.len(),
                cols: num_variables+1
            };
            
            let params=linear_regression(&m_x,&y);

            let regressor=LinearRegressor { parameters: params };

            return Ok(regressor);
        }
        else
        {
            return Err("No data cases.".to_string());
        }
    }

    /// Perform regression for a value
    /// 
    /// # Arguments
    /// 
    /// * `x` - Value for which to calculate the regression
    pub fn get_value(&self,x:&Vec<f64>) -> f64
    {
        let mut res:f64=0.0;
        for i in 0..x.len()
        {
            res+=self.parameters[i]*x[i];
        }
        res+=self.parameters[self.parameters.len()-1];
        return res;
    } 
}