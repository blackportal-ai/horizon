use std::env;
use std::path::PathBuf;

use lazy_static::lazy_static;

use horizon_common::HorizonCliState;

// get directories; default to ./data and ./config if env vars don't exist or aren't set
// otherwise get HORIZON_DATA_DIRECTORY and HORIZON_CONFIG_DIRECTORY from env vars

lazy_static! {
    static ref HORIZON_DATA_DIRECTORY: PathBuf = {
        match env::var("HORIZON_DATA_DIRECTORY") {
            Ok(val) => PathBuf::from(val),
            Err(_) => PathBuf::from("./data"),
        }
    };
    static ref HORIZON_CONFIG_DIRECTORY: PathBuf = {
        match env::var("HORIZON_CONFIG_DIRECTORY") {
            Ok(val) => PathBuf::from(val),
            Err(_) => PathBuf::from("./config"),
        }
    };
}

#[derive(Debug)]
pub struct JobCommand {}

impl JobCommand {
    pub async fn init_command() {
        let state = HorizonCliState::new(
            HORIZON_DATA_DIRECTORY.to_path_buf(),
            HORIZON_CONFIG_DIRECTORY.to_path_buf(),
        );

        state.init_data_folder().await;
        state.init_config_folder().await;
    }
}
