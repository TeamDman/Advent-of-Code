pub mod profiler;

// pub use profiler::time_function;
pub mod prelude {
    pub use crate::time_function;
    pub use crate::profiler::log_function_execution;
}
