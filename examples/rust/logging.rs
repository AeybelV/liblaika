use chrono::Utc;
use laika::logger::Logger;
use serde::Serialize;

#[derive(Serialize)]
struct LogEntry {
    message: String,
    timestamp: String,
}

fn main() -> std::io::Result<()> {
    // Initialize the logger with a directory and shard configuration
    let logger = Logger::initialize_logger("./logs", 16)?;

    // Create a log entry to serialize
    let entry = LogEntry {
        message: "System started".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    // Log the entry
    let log_file_path = logger.log(&entry)?;
    println!("Log file created at: {}", log_file_path);

    match logger.verify_and_repair_log(log_file_path.to_string()) {
        Ok(_) => {
            println!("Recovered log succesfully")
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }

    Ok(())
}
