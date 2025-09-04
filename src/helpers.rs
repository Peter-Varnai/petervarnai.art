use std::{
    collections::HashMap,
    env::{self, VarError},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::models::ConfigValue;

pub fn load_local_env_file() {
    if env::current_exe().unwrap().ends_with("release") {
        return;
    }

    let file = File::open(".env").expect(".env file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines/comments
        }
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key.trim(), value.trim());
        }
    }
}

pub fn server_config() -> Result<HashMap<String, ConfigValue>, VarError> {
    load_local_env_file();

    let mut config = HashMap::new();

    let host = ConfigValue::StringValue(env::var("HOST")?);
    let port = ConfigValue::NumberValue(
        env::var("PORT")?
            .parse()
            .expect("failed to parse PORT value"),
    );

    println!("app running at {:?}:{:?}", host, port);

    config.insert("host".to_string(), host);
    config.insert("port".to_string(), port);

    let root_dir = PathBuf::from(env::var("ROOT_DIR")?);

    let db_path = env::var("DB")?;
    let db_url = ConfigValue::PathValue(root_dir.clone().join(db_path));

    println!("connecting to db on the following address: {:?}", db_url);
    config.insert("db".to_string(), db_url);

    let password = ConfigValue::StringValue(env::var("PWD")?);
    config.insert("pwd".to_string(), password);

    config.insert("root_dir".to_string(), ConfigValue::PathValue(root_dir));

    Ok(config)
}
