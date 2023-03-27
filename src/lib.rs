pub mod utils;

//^(?=.*query_name).*?,([^\s]+)$
pub mod service{
  use std::{error::Error, io::BufRead};
  use crate::utils::{config::Config, io::IO};
  use regex::{Regex, escape};
  pub fn find_module(config: &Config)->Result<Vec<String>, Box<dyn Error>>{
    let mut result: Vec<String> = Vec::new();
    let query = escape(&config.get_query_name());
    let reg_query = if !config.is_strict() {
      format!("{query}.*")
    } else {
      format!("{delimiter}{query}",delimiter=escape(config.get_delimiter()))
    };
    let reg_str = format!("^.*{reg_query},([^\\s]+)$");
    let reg: Regex = Regex::new(reg_str.as_str()).unwrap();
    let mut buf_io = IO::new(config.get_file_path())?;
    let buffer_reader = buf_io.get_buffer_reader();
    let lines = buffer_reader.lines();
    for line in lines{
      if let Ok(val) = line{
        if let Some(capture) = reg.captures(val.as_str()){ 
          if let Some(group) = capture.get(1){
            result.push(group.as_str().to_string());
          }

        }
      }
    }
    Ok(result)
  }
}