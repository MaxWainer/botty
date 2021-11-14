use toml::Value;
use crate::MySQLCredentials;

#[derive(Deserialize)]
pub struct Config {
    token: String,
    mysql_credentials: MySQLCredentials
}

pub fn read_config(file_name: String) -> Config {
    let result = std::fs::read_to_string(file_name)
        .expect("An error acquired while reading file!");

    let config: Config = toml::to_string(result.as_str())
        .expect("Error while parsing file!")
        .unwrap();


}