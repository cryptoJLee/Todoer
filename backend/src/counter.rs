use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
  pub count: i32
}

impl Counter {
  fn get_redis_url() -> String {
    let config = Config::new();
    config.map.get("REDIS_URL")
        .unwrap().as_str()
        .unwrap().to_owned()
  }

  pub fn save(self) -> Result<(), redis::RedisError> {
    let serialized = serde_yaml::to_vec(&self).unwrap();
    let client = redis::Client::open(
      Counter::get_redis_url()
    )?;
    let mut con = client.get_connection()?;
    match redis::cmd("SET").arg("COUNTER")
                           .arg(serialized)
                           .query::<()>(&mut con) {
      Ok(_) => Ok(()),
      Err(error) => {
        println!("{:?}", error);
        Err(error)
      }
    }
  }

  pub fn load() -> Result<Counter, redis::RedisError> {
    let client = redis::Client::open(
      Counter::get_redis_url()
    )?;
    let mut con = client.get_connection()?;
    let byte_data: Vec<u8> = redis::cmd("GET")
        .arg("COUNTER")
        .query(&mut con)?;
    Ok(serde_yaml::from_slice(&byte_data).unwrap())
  }
}