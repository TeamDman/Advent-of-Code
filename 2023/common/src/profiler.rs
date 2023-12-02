// Logging function
pub fn log_function_execution(
    function_name: &'static str,
    input_length: usize,
    duration: std::time::Duration,
) {
    use std::fs::OpenOptions;
    use std::io::Write;

    // Define a struct for the log data
    #[derive(serde::Serialize)]
    struct LogData {
        timestamp: i64,
        mode: &'static str,
        function_name: &'static str,
        input_length: usize,
        duration_nanoseconds: u128,
    }

    let log_data = LogData {
        timestamp: chrono::Utc::now().timestamp(),
        mode: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
        function_name,
        input_length,
        duration_nanoseconds: duration.as_nanos(),
    };

    let log_message = serde_json::to_string(&log_data).expect("JSON serialization failed");
    let log_message = format!("{}\n", log_message); // JSONL format

    // Write to file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("function_times.log")
        .expect("Unable to open file");
    Write::write_all(&mut file, log_message.as_bytes()).expect("Unable to write data");

    println!("Logged JSON data for {}: {:?}", function_name, duration);
}

// Updated macro
#[macro_export]
macro_rules! time_function {
    ($func:ident, $input:expr $(, $args:expr)*) => {{
        let start = std::time::Instant::now();
        let result = $func($input $(, $args)*);
        let duration = start.elapsed();

        // Call the logging function
        crate::log_function_execution(stringify!($func), $input.len(), duration);
    }};
}
