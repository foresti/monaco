pub mod matrix;
pub mod math;
pub mod linear_regressor;

#[cfg(test)]
mod tests
{
    use crate::math;
    use crate::matrix;
    use crate::matrix::Matrix;

    #[test]
    fn chol_one_element()
    {
        let m=vec![4.0];
        let chol:Vec<f64>=matrix::cholesky(&m);
        assert!(f64::abs(chol[0]-(2.0))<0.001);
    }

    #[test]
    fn invcdf99() 
    {
        let x1 = math::normal_invcdf(0.5);
        assert!(f64::abs(x1-0.0)<0.001);
        let x2 = math::normal_invcdf(0.99);
        assert!(f64::abs(x2-2.3263478740408408)<0.001);
        let x3 = math::normal_invcdf(0.999);
        assert!(f64::abs(x3-3.090232306167813)<0.001);

        let x4 = math::normal_invcdf(0.5);
        assert!(f64::abs(x4-(-x1))<0.001);
        let x5 = math::normal_invcdf(0.01);
        println!("x5: {}, x2: {}",x5,x2);
        assert!(f64::abs(x5-(-x2))<0.001);
        let x6 = math::normal_invcdf(0.001);
        assert!(f64::abs(x6-(-x3))<0.001);

        let x7 = math::normal_invcdf(0.1);
        assert!(f64::abs(x7-(-1.2815515655446004))<0.001);
        let x8 = math::normal_invcdf(0.25);
        assert!(f64::abs(x8-(-0.6744897501960817))<0.001);
        let x9 = math::normal_invcdf(0.4);
        assert!(f64::abs(x9-(-0.2533471031357997))<0.001);

    }

    #[test]
    fn inv() 
    {
        let m:Matrix=Matrix
        {
            data:vec![1.0,0.3,0.8,0.3,1.0,-0.3,0.8,-0.3,1.0],
            rows:3,
            cols:3
        };
        let inv=m.inverse();
        assert!(inv.data.len()==9);
        assert!(f64::abs(inv.data[0]-(25.27777778))<0.001);
    }

    #[test]
    fn mult() 
    {
        let matrix_a = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
        let matrix_b = vec![1.0,2.0,3.0,4.0,5.0,6.0];
        let c=matrix::multiply(&matrix_a, 3, 3, &matrix_b, 3, 2);
        assert!(c.len()==6);
        assert!(f64::abs(c[0]-(22.0))<0.001);
        assert!(f64::abs(c[1]-(28.0))<0.001);
        assert!(f64::abs(c[2]-(49.0))<0.001);
        assert!(f64::abs(c[3]-(64.0))<0.001);
        assert!(f64::abs(c[4]-(76.0))<0.001);
        assert!(f64::abs(c[5]-(100.0))<0.001);
    }

    #[test]
    fn cholesky()
    {
        let m:Vec<f64>=vec![1.0,0.3,0.8,0.3,1.0,-0.3,0.8,-0.3,1.0];
        let chol:Vec<f64>=matrix::cholesky(&m);
        assert!(f64::abs(chol[0]-(1.0))<0.001);
        assert!(f64::abs(chol[1]-(0.0))<0.001);
        assert!(f64::abs(chol[2]-(0.0))<0.001);
        assert!(f64::abs(chol[3]-(0.3))<0.001);
        assert!(f64::abs(chol[4]-(0.9539392))<0.001);
        assert!(f64::abs(chol[5]-(0.0))<0.001);
        assert!(f64::abs(chol[6]-(0.8))<0.001);
        assert!(f64::abs(chol[7]-(-0.56607381))<0.001);
        assert!(f64::abs(chol[8]-(0.19889806))<0.001);
    }
}