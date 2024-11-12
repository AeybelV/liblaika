use chrono::Utc;
use reed_solomon::{Decoder, Encoder};
use serde::Serialize;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct Logger {
    dir_path: String,
    encoder: Encoder, // Reed-Solomon codec for ECC
    ecc_len: usize,
}

impl Logger {
    pub fn initialize_logger(dir_path: &str, ecc_len: usize) -> io::Result<Self> {
        fs::create_dir_all(dir_path)?; // Ensure directory exists
        let encoder = Encoder::new(ecc_len);
        Ok(Self {
            dir_path: dir_path.to_string(),
            encoder,
            ecc_len,
        })
    }

    pub fn log<T: Serialize>(&self, entry: &T) -> io::Result<String> {
        // Serialize the entry
        let serialized_entry = serde_json::to_string(entry)?;

        // Encode data and parity shards using Reed-Solomon
        let data: &[u8] = serialized_entry.as_bytes();
        let shards = self.encoder.encode(data);

        // Generate timestamped filename
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let file_path = format!("{}/log_{}.log", self.dir_path, timestamp);

        // Write log data to file
        let mut file = File::create(&file_path)?;
        file.write_all(shards.as_ref())?;
        file.sync_all()?;
        Ok(file_path)
    }

    pub fn verify_and_repair_log(&self, file_path: String) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let dec = Decoder::new(self.ecc_len);

        // Try to recover data
        let known_erasures = [0];
        let correction = dec.correct(&mut contents, Some(&known_erasures));
        match correction {
            Ok(recovered_data) => {
                let recv_str = std::str::from_utf8(recovered_data.data()).unwrap();
                file.seek(SeekFrom::Start(0))?;
                file.set_len(0)?;
                file.write_all(recv_str.as_bytes())?;
                Ok(())
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed recovering log",
            )),
        }
    }
}
