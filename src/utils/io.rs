use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::{Error,BufReader, ErrorKind, Read};
use std::path::{Path, self};

pub struct IO{
  buffer_reader: BufReader<File>,
}
impl IO{
  pub fn new(file_path: &str)->Result<IO, Error>{
    let file =match File::open(file_path) {
        Ok(file) => file,
        Err(err) =>{
          match err.kind() {
            ErrorKind::NotFound=>{
              return Err(Error::new(ErrorKind::NotFound, "File Not Found"))
            }
            ErrorKind::PermissionDenied=>{
              return Err(Error::new(ErrorKind::NotFound, "Cannot Access the File"))
            }
            err=>{
              return Err(Error::new(err, "There are some error when open file!"))
            }
          }
        }
    }; 
    let buffer_reader = BufReader::new(file);
    Ok(IO {buffer_reader})
  }
  
  pub fn get_buffer_reader(&mut self)->&mut BufReader<File>{
    &mut self.buffer_reader
  }
}
pub fn detect_path_separator(file: &File)->char{
  let mut buff_reader = BufReader::new(file);
  let mut buffer : [u8;255] = [0;255];
  while let Ok(_) = buff_reader.read(&mut buffer) {
      if buffer.contains(&('\\' as u8)){
        return '\\'
      }else if buffer.contains(&('/' as u8))  {
          return '/'
      }
  }
  return '_';
}
pub fn handle_file_recursive<F>(result: &mut HashMap::<String,Vec<String>>, query_reg:&String, path:&Path, handle: &F)
where F: Fn(&mut HashMap::<String,Vec<String>>,&String,&str)->Result<(), Error>{
  if path.is_file() {
    handle(result,query_reg,path.to_str().unwrap()).unwrap_or_else(|e|{
      println!("Error when processing file: {}", e.to_string());
    });
    return;
  } else if !path.is_dir(){
    return;
  }
  if let Ok(read_dir) = read_dir(path){
      for entry_dir in read_dir.into_iter().flatten(){
        if !entry_dir.metadata().is_ok(){
            continue;
        }
        let meta: std::fs::Metadata = entry_dir.metadata().unwrap();
        let mut full_file_path: String = String::new();
        full_file_path.push_str(path.to_str().unwrap_or_default());
        full_file_path += path::MAIN_SEPARATOR.to_string().as_str();
        full_file_path+= entry_dir.file_name().to_str().unwrap();
        if meta.is_dir() {
            handle_file_recursive(result,query_reg,Path::new(&full_file_path), handle);
        }
        else{
          if let Err(_) = handle(result,query_reg,&full_file_path){
            return;
          }   
        }
      }
  }
  ()
}
