use mysql::serde_json::Value::String;
use std::fs;
use crate::config_parser::read_config;
use crate::mysql::{MySQLCredentials, open_connection};

mod mysql;
mod config_parser;

#[tokio::main]
async fn main() {

    let config = read_config(String::from("config.toml"))

}

async fn execute_command() {

}
