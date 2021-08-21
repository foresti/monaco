#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug 
{
    //($x:expr) => { dbg!($x) }
    ($x:expr) => { println!("{}",$x) }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug 
{
    ($x:expr) => { std::convert::identity($x) }
}