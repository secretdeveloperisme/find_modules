#[derive(Debug)]
pub struct Config<'a>{
  delimiter: &'a str,
  query_file_name: &'a str, 
  input_file_path: &'a str,
  is_strict: bool
}

impl <'a>Config<'a>{
    pub fn new() ->Self {
      Config{
        delimiter: "\\",
        query_file_name: "",
        input_file_path : "",
        is_strict: false
      }
    }
    pub fn set_file_path(&mut self,file_path : &'a str){
      self.input_file_path = file_path;
    }
     pub fn delimiter(&mut self,delimiter : &'a str){
      self.delimiter = delimiter;
    }
    pub fn set_query_name(&mut self,query_file : &'a str){
      self.query_file_name = query_file;
    }
    pub fn set_strict(&mut self,strict: bool){
      self.is_strict = strict;
    }
    pub fn is_ok(&self)->bool{
     if self.query_file_name.len() > 0 && self.input_file_path.len()> 0 {true} else {false}
    }
    pub fn get_file_path(&self)->&str{
      self.input_file_path
    }
    pub fn get_query_name(&self)->&str{
      self.query_file_name
    }
    pub fn is_strict(&self)->bool{
      self.is_strict
    }
    pub fn get_delimiter(&self)->&str{
      self.delimiter
    }
    
}