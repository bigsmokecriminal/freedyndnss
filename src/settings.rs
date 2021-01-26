
use config::{Config, File, ConfigError};


#[derive(Debug, Deserialize, Clone)]
pub struct Domain {
    pub domain : String,
    pub pass : u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUrl {
    pub url : String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub url : UpdateUrl,
    pub domains : Vec<Domain>,
}

impl Settings {
    pub fn from_file(config_path : &String) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(config_path))?;
        s.try_into()
    }

   
}