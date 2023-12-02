#[macro_export]
macro_rules! time_function {
    ($func:ident $(, $args:expr)*) => {{
        let start = std::time::Instant::now();
        $func($($args),*);
        let duration = start.elapsed();
        println!("Time elapsed in {}() is: {:?}", stringify!($func), duration);
    }};
}