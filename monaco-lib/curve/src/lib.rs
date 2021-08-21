pub mod curve;

#[cfg(test)]
mod tests {

    use crate::curve;
    use crate::curve::Curve;
    use crate::curve::IrCurve;

    #[test]
    fn exp_discount() {
        let c:Curve=Curve {
            data: vec![(0.001,0.02),(0.1,0.0025),(0.5,0.03),(1.0,0.028)]
        };

        let df:f64=c.get_cont_df(0.8).unwrap();
        assert!(f64::abs(df-0.9772233940)<0.001);
    }
}