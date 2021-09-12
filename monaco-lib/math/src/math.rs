extern crate rand;
use crate::matrix::*;

//extern crate lapack;
//use lapack::*;
//use macros::debug;
use rand::{thread_rng, Rng};
use rand::distributions::Open01;

/// Compute the parameters for linear regression from two vectors
///
///  # Arguments
/// 
/// * `x` - X values (n x m)
/// * `y` - Y values (n x 1)
pub fn linear_regression(x:&Matrix,y:&Vec<f64>) -> Vec<f64>
{
    let a=x.transpose().multiply(&x);
    let b=a.inverse();
    let c=b.multiply(&x.transpose());
    let d=multiply(&c.data, c.rows, c.cols, &y, y.len(), 1);

    return d;
}
/// Compute the parameters for linear regression from two vectors (the x vector contains only a single variable)
///
///  # Arguments
/// 
/// * `x` - X values (n x 1)
/// * `y` - Y values (m x 1)
pub fn linear_regression_one_var(data:&Vec<(f64,f64)>) -> (f64,f64)
{
    let mut sum_x:f64=0.0;
    let mut sum_y:f64=0.0;
    let mut sum_xy:f64=0.0;
    let mut sum_xx:f64=0.0;
    let mut n:f64=0.0;

    for i in 0..data.len()
    {
        n=n+1.0;
        sum_x=sum_x+data[i].0;
        sum_y=sum_y+data[i].1;
        sum_xy=sum_xy+data[i].0*data[i].1;
        sum_xx=sum_xx+data[i].0*data[i].0;
    }
    
    let beta=(n*sum_xy-sum_x*sum_y)/(n*sum_xx-(sum_x*sum_x));
    let alpha=(sum_y/n)-beta*(sum_x/n);

    return (alpha,beta)
}

/// Perform linear interpolation
/// 
/// # Arguments
/// 
/// * `data` - Vector of (x,y) tuples
/// * `x` - value for which to provide a corresponding y value
pub fn interpolate(data:&Vec<(f64,f64)>, x:f64) -> f64
{
    if data.len()==0
    {
        panic!("Data vector empty.");
    }
    if x <= data[0].0
    {
        return data[0].1;
    }
    if x >= data[data.len() - 1].0
    {
        return data[data.len() - 1].1;
    }

    let up:usize;
    let down:usize;
    let pos_result=data.binary_search_by(|val| val.0.partial_cmp(&x).expect("NaN"));
    
    match pos_result
    {
        Ok(p)   =>  {up=p; down=p;},
        Err(p)  =>  {up=p; down=p-1}    
    }
    if up!=down
    {
        return ((data[up].1 - data[down].1) / (data[up].0 - data[down].0)) * (x - data[down].0) + data[down].1;
    }
    else
    {
        return data[up].1;
    }
}

/// Simulate normally distributed vectors of correlated variates
///
/// # Arguments
/// 
/// * `num_var` - Number of random variables to simulate
/// * `sample_size` - Number of correlated vector to produce
/// * `correlation_matrix` - Correlation matrix (dimensions: num_var x num_var)
/// 
/// # Returns
/// 
/// Matrix in vector form (row by row) of dimensions: sample_size x num_var
/// To retrieve variable j for sample i  in the matrix: matrix[i*num_var+j]
pub fn simulate_normal_variates(num_var:usize, sample_size:usize, correlation_matrix: &Vec<f64>) -> Vec<f64>
{
    let mut result = vec![0.0; sample_size*num_var];
    let mut rng = thread_rng();

    for i in 0..sample_size
    {
        for j in 0..num_var
        {
            let p:f64=rng.sample(Open01);
            let normal_val=normal_invcdf(p);
            result[i*num_var+j]=normal_val;
        }
    }

    let chol=cholesky(&correlation_matrix);
    let chol_transposed=transpose(&chol, num_var,num_var);
    let x=multiply(&result, sample_size, num_var, &chol_transposed, num_var, num_var);
    let y=transpose(&x, sample_size, num_var);

    return y;
}

/// Compute the inverse CDF from a normal distribution
/// 
/// # Arguments
/// 
/// *`p` - Probability
pub fn normal_invcdf(p: f64)-> f64
{
    if p<0.5
    {
        return -normal_g(-p);
    }
    else
    {
        return normal_g(1.0-p);
    }
}

fn normal_g(p: f64) -> f64
{
    let t=f64::sqrt(f64::ln(1.0/(f64::powf(p,2.0))));
    let c0=2.515517;
    let c1=0.802853;
    let c2=0.010328;
    let d1=1.432788;
    let d2=0.189269;
    let d3=0.001308;

    let num=c0+c1*t+c2*f64::powf(t,2.0);
    let denom=1.0+d1*t+d2*f64::powf(t,2.0)+d3*f64::powf(t,3.0);

    let x=t-(num/denom);

    return x;
}