#[macro_export]
macro_rules! time_function {
    ($func:ident, $input:expr $(, $args:expr)*) => {{
        let start = std::time::Instant::now();
        $func($input $(, $args)*);
        let duration = start.elapsed();

        // Define a struct for the log data
        #[derive(serde::Serialize)]
        struct LogData {
            timestamp: i64,
            mode: &'static str,
            function_name: &'static str,
            input_length: usize,
            time_nanoseconds: u128,
        }
        

        let log_data = LogData {
            timestamp: chrono::Utc::now().timestamp(),
            mode: if cfg!(debug_assertions) { "debug" } else { "release" },
            function_name: stringify!($func),
            input_length: $input.len(), // Works for both &str and String
            time_nanoseconds: duration.as_nanos(),
        };

        let log_message = serde_json::to_string(&log_data).expect("JSON serialization failed");
        let log_message = format!("{}\n", log_message); // JSONL format

        // Write to file
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("function_times.log")
            .expect("Unable to open file");
        std::io::Write::write_all(&mut file, log_message.as_bytes())
            .expect("Unable to write data");

        println!("Logged JSON data for {}: {:?}", stringify!($func), duration);
    }};
}