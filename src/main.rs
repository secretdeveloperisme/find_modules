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
  let results = service::find_module(&config);
  if let Err(err) = results {
    eprintln!("[Error]");
    eprintln!("There are some errors when running tool");
    eprintln!("Cause: {err}");
    process::exit(-1);
  }
  println!("[Result]");
  for result in results.unwrap(){
    println!("{}", result);
  }
  process::exit(0);
  
}