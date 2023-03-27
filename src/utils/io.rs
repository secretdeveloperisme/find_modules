use std::fs::{File};
use std::io::{Error,BufReader, ErrorKind};
pub struct IO{
  buffer_reader: BufReader<File>,
}
impl IO{
  pub fn new( file_path: &str)->Result<IO, Error>{
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
