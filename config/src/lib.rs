//! a bare bones configuration file template with minimal dependencies
//! suitable for customization to a variety of needs
use {
     serde::{Serialize, Deserialize},
     anyhow::{Result, anyhow, Context},
};

pub mod keypair;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn save(&self, file_path: &str) -> Result<()> {
       std::fs::write(file_path, serde_yaml::to_string(self)?).with_context(|| SAVE_FAILURE)?;
       Ok(())
    }
    pub fn load(file_path: &str) -> Result<()> {
        Ok(serde_yaml::from_str(&std::fs::read_to_string(file_path).with_context(|| LOAD_FAILURE)?).with_context(|| DESERIALIZE_FAILURE)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}


const SAVE_FAILURE: &str = "failed to save file";
const LOAD_FAILURE: &str = "failed to load file";
const DESERIALIZE_FAILURE: &str = "failed to deserialize";


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_config() {
        let conf_1 = Config::default();
        let conf_2 = Config::new();


        conf_1.save("conf_1").unwrap();
        conf_2.save("conf_2").unwrap();

        let conf_1 = Config::load("conf_1").unwrap();
        let conf_2 = Config::load("conf_2").unwrap();

        let conf_1_str = serde_yaml::to_string(&conf_1).unwrap();
        let conf_2_str = serde_yaml::to_string(&conf_2).unwrap();
        assert_eq!(conf_1_str, conf_2_str);
    }
}