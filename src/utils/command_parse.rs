use super::config::Config;

pub fn parse<'a>(args: &'a Vec<String>)->Result<Config<'a>, String>{
  let mut config = Config::new();
  let mut args_iter = args.iter();
  while let Some(value) = args_iter.next(){
    match value.as_str() {
        "-i" | "--input" =>{
          if let Some(next_val) = args_iter.next(){
            config.set_file_path(&next_val);
          }
        },
        "-n" | "--name" =>{
          if let Some(next_val) = args_iter.next(){
            config.set_query_name(&next_val);
          }
        }
        "-s" | "--strict" =>{
          config.set_strict(true);
        }
        "-j" | "--json" =>{
          config.set_json(true);
        }
        _ =>{}
    }
  }
  if config.is_ok(){
    Ok(config)
  }
  else{
    Err(String::from("Can not parse the arguments"))
  }
  
}
