use serde::{Serialize, Deserialize};
use serde_derive::{Deserialize, Serialize};
use std::fs::{OpenOptions, File};
use std::io::{self, Write, BufRead};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        Self { service, username, password }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("❌ Failed to serialize to JSON")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("❌ Error writing to file: {}", e);
                } else {
                    println!("✅ Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("❌ Error opening file: {}", e),
        }
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }
    Ok(services)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}