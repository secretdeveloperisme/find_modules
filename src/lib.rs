pub mod utils;

// regex using look ahead: ^(?=.*query_name).*?,([^\s]+)$
pub mod service{
  use std::{io::Error, io::BufRead, path::Path, collections::HashMap};
  use crate::utils::{config::Config, io::{IO,handle_file_recursive}};
  use regex::{Regex, escape};
  use serde_json;
  pub fn find_module(config: &Config){
    let mut result = HashMap::<String,Vec<String>>::new();
    let query = escape(&config.get_query_name());
    let reg_query = if !config.is_strict() {
      format!("{query}.*")
    } else {
      format!("{delimiter}{query}",delimiter=r"[\\/]")
    };
    let file_path: &Path = Path::new(config.get_file_path());
    handle_file_recursive(&mut result,&reg_query, file_path,&handle_find_target);
    if result.len() == 0 {
      println!("NOT_FOUND");
    }
    if config.is_json() {
      println!("{}",to_json(&result).unwrap());
    }
    else {
      println!("{}",to_format_output(&result));
    }
  }
  fn handle_find_target(result: &mut HashMap::<String,Vec<String>>, reg_query:&String, path: &str)->Result<(), Error>{
    let reg_str: String = format!("^.*{reg_query},([^\\s]+)$");
    let reg: Regex = Regex::new(reg_str.as_str()).unwrap();
    let mut buf_io = IO::new(path)?;
    let buffer_reader = buf_io.get_buffer_reader();
    let lines = buffer_reader.lines();
    for line in lines{
      if let Ok(val) = line{
        if let Some(capture) = reg.captures(val.as_str()){ 
          if let Some(group) = capture.get(1){
            if result.get(path).is_none(){
              let mut paths = Vec::<String>::new();
              paths.push(group.as_str().to_string());
              result.insert(path.to_string(), paths);
            }
            else{
              let paths = result.get_mut(path).unwrap();
              paths.push(group.as_str().to_string());
            }
          }
        }
      }
    }
    Ok(())
  }
  fn to_json<T: serde::ser::Serialize>(target: &T)->Result<String, serde_json::Error>{
    serde_json::to_string(target)
  }
  fn to_format_output(target: &HashMap::<String,Vec<String>>)->String{
    let mut result = String::new();
    for pair in target{
     result += &format!("[{}]\n", pair.0);
      for item in pair.1{
        result += &format!("{}\n", item);
      }
    }
    result
  }
}