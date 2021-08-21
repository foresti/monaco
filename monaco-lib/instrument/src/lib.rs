pub mod instrument;
pub mod lsm;
pub mod vanilla_swap;
pub mod callable_swap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
