# find_modules

## Find the respective module location on the server

Usage: find_module.exe [OPTION] ... 
  
  Mandatory arguments to long options are mandatory for short options too.

  *-i, --input*                  input file or folder path

  *-n, --name*                  module name

  *-s, --strict*                find module name 

  with absolute name
  *-j, --json*                   display result with json format 

  Example:

  ```bash
  find_module.exe -i data\input_win -n util --json
  ```

  > Note: Each line in data file is specified by format: `source_path,destination_path`\
  Example: **foo\abc\module.exe;bar\abc\module.exe**



