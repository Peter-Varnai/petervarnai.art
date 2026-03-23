use std::{
    env::{self, VarError},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db: PathBuf,
    pub pwd: String,
    pub root_dir: PathBuf,
}

pub fn load_local_env_file() {
    println!("checking enviroment variables");
    if !cfg!(debug_assertions) {
        return;
    }

    let file = File::open(".env").expect(".env file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key.trim(), value.trim());
        }
    }
}

pub fn server_config() -> Result<ServerConfig, VarError> {
    load_local_env_file();

    let host = env::var("HOST")?;
    let port = env::var("PORT")?
        .parse()
        .expect("failed to parse PORT value");

    println!("app running at {:?}:{:?}", host, port);

    let root_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let db_path = env::var("DB")?;
    let db = PathBuf::from(&db_path);
    let db = if db.is_absolute() {
        db
    } else {
        root_dir.join(db_path)
    };

    println!("connecting to db on the following address: {:?}", db);

    let pwd = env::var("APP_PASSWORD")?;

    Ok(ServerConfig {
        host,
        port,
        db,
        pwd,
        root_dir,
    })
}
