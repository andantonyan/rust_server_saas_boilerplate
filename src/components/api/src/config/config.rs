use std::env;

const CONFIG_FILE_ENV_KEY: &'static str = "CONFIG_FILE_PATH";

trait ConfigTrait {
  fn set_config_root(&mut self, env_key: &'static str) {}
  fn parse_config_file(&self) {}
}

pub struct Config {
  path: String
}

impl Config {
  fn new() -> Config {
    Config {
      path: "".to_string()
    }
  }
}

impl ConfigTrait for Config {
  fn set_config_root(&mut self, env_key: &'static str) {
    match env::var(env_key) {
      Ok(val) => self.path = val,
      Err(e) => {
        println!("couldn't interpret {}: {} \nUsing default value", env_key, e);
        self.path = "./config.toml".to_string();
      },
    }
  }

  fn parse_config_file(&self) {}
}

pub fn main() {
  let mut config = Config::new();
  config.set_config_root(CONFIG_FILE_ENV_KEY);
  println!("Config path is {}", config.path);
}
