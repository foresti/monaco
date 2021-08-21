use math::matrix::Matrix;
use math::linear_regressor::LinearRegressor;

fn main() {
    println!("Start...");
    let x:Matrix=Matrix
    {
        data:vec![0.3,0.8,1.5,2.1,2.8,5.9,10.2,15.2],
        rows:4,
        cols:2
    };
    let y:Matrix=Matrix
    {
        data:vec![3.0,4.2,10.2,8.8],
        rows:4,
        cols:1
    };
    println!("Creating regressor...");
    let r=LinearRegressor::create_regressor(&x.data,&y.data);
    println!("Regressor created.");

    let num_params=x.cols+1;
    for i in 0..num_params
    {
        println!("{}: {}",i,r.parameters[i]);
    }
}