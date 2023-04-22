
use std::{env::args, process};
use find_module::service;

use find_module::utils::command_parse::parse;
fn main(){
  let arg_vec: Vec<String>= args().collect();
  let config = match parse(&arg_vec){
    Ok(config) => config,
    Err(msg)=>{
      eprintln!("{msg}");
      process::exit(-1);
    }
  };
  service::find_module(&config);
  process::exit(0);
}