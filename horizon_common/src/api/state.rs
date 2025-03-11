use std::path::PathBuf;
use tokio::fs::create_dir_all;

#[derive(Debug)]
pub struct HorizonState {
    data_folder: PathBuf,

    config_folder: PathBuf,
}

impl HorizonState {
    pub fn new(data_folder: PathBuf, config_folder: PathBuf) -> Self {
        Self {
            data_folder,
            config_folder,
        }
    }

    pub async fn init_data_folder(&self) {
        if !self.data_folder.exists() {
            create_dir_all(&self.data_folder).await.unwrap();
        }
    }

    pub async fn init_config_folder(&self) {
        if !self.config_folder.exists() {
            create_dir_all(&self.config_folder).await.unwrap();
        }
    }
}
