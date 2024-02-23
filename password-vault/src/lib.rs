use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, stdin, stdout};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    #[allow(dead_code)]

    pub fn from_user_input() -> Self {
        println!("Enter service name: ");
        let mut service = String::new();
        stdin()
            .read_line(&mut service)
            .expect("Failed to read service name");

        println!("Enter username: ");
        let mut username = String::new();
        stdin()
            .read_line(&mut username)
            .expect("Failed to read username");

        println!("Enter password: ");
        let mut password = String::new();
        stdin()
            .read_line(&mut password)
            .expect("Failed to read password");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }

    pub fn write_to_file(&self){
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => {
                file.write_all(json_output.as_bytes())
                    .expect("Failed to write to file");
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        }
    }

    pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, Error>{
        let file = File::open("passwords.json")?;
        let reader = std::io::BufReader::new(file);
        let mut services = Vec::new();

        for line in reader.lines(){
            let service = line?;
            let service_info = ServiceInfo::from_json(&service)?;
            services.push(service_info);
        }

        Ok(services)
    }

    pub fn prompt(prompt: &str) -> String{
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn delete_password_entry(service: &str) -> Result<(), Error>{
        let mut services = ServiceInfo::read_passwords_from_file()?;
        services.retain(|service_info| service_info.service != service);

        let updated_json = serde_json::to_string(&services)?;

        let path = Path::new("passwords.json");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(updated_json.as_bytes())?;

        Ok(())
    }
}

