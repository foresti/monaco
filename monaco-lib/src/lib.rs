pub use model;
pub use instrument;
pub use exposure_simulation;
pub use data_cube;
pub use curve;
pub use logger;

pub mod testmod
{
   pub struct TestStruct
   {
       pub t:f64
   }
}

#[cfg(test)]
pub mod tests
{
    use model::hw1f::Hw1f;
    #[test]
    fn create_hw1f() {
        Hw1f{
            name:"Test_Hw1f".to_string(),
            initial_rate:0.02,
            term_structure: vec![(0.1,0.1)],
            thetas: vec![(0.1,0.1)],
            a: vec![(0.1,0.1)],
            sigmas: vec![(0.1,0.1)],
         };
        assert_eq!(2 + 2, 4);
    }
}
