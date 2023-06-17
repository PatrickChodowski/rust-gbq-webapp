use serde::{Serialize, Deserialize};
use std::fs::{self};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dbname: String
}

impl Config {
    pub fn load() -> Config{
        let path: &str = "./config/config.json";
        let data: String = read_txt(path);
        return serde_json::from_str(&data).expect(&format!("\n [ERROR utils.config] Unable to get data from {path} \n"));
    }

}

pub fn read_txt(file_path: &str) -> String {
    let data: String = fs::read_to_string(file_path)
                          .expect(&format!("\n [ERROR utils.read_txt] Unable to read file {file_path}  \n"));
    return data;
  }
  