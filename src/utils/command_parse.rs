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
        "-c" | "--case_sensitive" =>{
          config.set_case_sensitive(true);
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
pub fn usage(){
  let output = String::from(r#"
  Usage: find_module.exe [OPTION] ... 
  Find the respective module location on the server

  Mandatory arguments to long options are mandatory for short options too.
  -i, --input                  input file or folder path
  -n, --name                   module name
  -s, --strict                 find module name with absolute name
  -j, --json                   display result with json format
  -c, --case_sensitive         search with case sensitive, default is false 
  Example:
  find_module.exe -i data\input_win -n util --json
  "#);
  println!("{}", output);
  
}
