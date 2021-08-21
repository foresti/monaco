use math::matrix::Matrix;

fn main() {
    println!("Start...");
    let mat:Matrix=Matrix
    {
        data:vec![1.0,0.3,0.8,0.3,1.0,-0.3,0.8,-0.3,1.0],
        rows:3,
        cols:3
    };
    let inv=mat.inverse();
    for i in 0..3
    {
        for j in 0..3
        {
            println!("{}/{}: {}",i,j,inv.data[i*inv.cols+j]);
        }
    }
}