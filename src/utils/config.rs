use std::path::Path;

#[derive(Debug)]
pub struct Config<'a>{
  delimiter: &'a str,
  query_file_name: &'a str, 
  input_file_path: &'a str,
  type_of_file: TypeFile,
  is_strict: bool,
  case_sensitive: bool,
  is_json: bool
}
#[derive(Debug)]
pub enum TypeFile{
  FOLDER, FILE, INVALID
}

impl <'a>Config<'a>{
    pub fn new() ->Self {
      Config{
        delimiter: "\\",
        query_file_name: "",
        input_file_path : "",
        is_json: false,
        case_sensitive: false,
        type_of_file: TypeFile::INVALID,
        is_strict: false
      }
    }
    pub fn set_file_path(&mut self,file_path : &'a str){
      self.input_file_path = file_path;
      self.set_type_of_file();
    }
    fn set_type_of_file(& mut self){
      let path = Path::new(self.input_file_path);
      if path.is_file(){
        self.type_of_file = TypeFile::FILE
      } else {
        self.type_of_file = TypeFile::FOLDER
      }
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
    pub fn set_json(&mut self,json: bool){
      self.is_json = json;
    }
    pub fn set_case_sensitive(&mut self,ignore_case: bool){
      self.case_sensitive = ignore_case;
    }
    pub fn is_ok(&self)->bool{
     if self.query_file_name.len() > 0 
     && (self.input_file_path.len()> 0)
     {true} else {false}
    }
    pub fn get_file_path(&self)->&str{
      self.input_file_path
    }
    pub fn get_type_of_file(&self)->&TypeFile{
      &self.type_of_file
    }
    pub fn get_query_name(&self)->&str{
      self.query_file_name
    }
    pub fn is_strict(&self)->bool{
      self.is_strict
    }
    pub fn is_json(&self)->bool{
      self.is_json
    }
    pub fn case_sensitive(&self)->bool{
      self.case_sensitive
    }
    pub fn get_delimiter(&self)->&str{
      self.delimiter
    }    
    
}