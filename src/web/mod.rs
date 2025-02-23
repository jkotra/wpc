use std::{error::Error, path::PathBuf};

use async_trait::async_trait;

use crate::settings::WPCSettings;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PluginDetails {
    pub name: String,
    pub version: String,
    pub author: String,
}

#[async_trait]
pub trait WPCPlugin {
    fn details(&self) -> PluginDetails;
    async fn init(&mut self, config_file: Option<PathBuf>) -> Result<(), Box<dyn Error>>;
    async fn init_from_settings(&mut self, settings: WPCSettings) -> Result<(), Box<dyn Error>>;
    async fn update(&self, savepath: &PathBuf, maxage: i64) -> Vec<String>;
}

pub mod reddit;
pub mod wallhaven;
