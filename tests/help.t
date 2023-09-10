Modowners is a tool to programatically update CODEOWNERS files.
It provides subcommands to add and remove code owners:

  $ modowners --help
  Usage: modowners <COMMAND>
  
  Commands:
    add     Add a code owner
    remove  Remove a code owner
    help    Print this message or the help of the given subcommand(s)
  
  Options:
    -h, --help     Print help
    -V, --version  Print version

  $ modowners add --help
  Add a code owner
  
  Usage: modowners add <PATTERN> <OWNER>
  
  Arguments:
    <PATTERN>  
    <OWNER>    
  
  Options:
    -h, --help  Print help

  $ modowners remove --help
  Remove a code owner
  
  Usage: modowners remove <PATTERN> <OWNER>
  
  Arguments:
    <PATTERN>  
    <OWNER>    
  
  Options:
    -h, --help  Print help
