Modowners is a tool to programatically update CODEOWNERS files.
It provides subcommands to add and remove code owners:

Test adding code owners with modowners.

It should create the CODEOWNERS file if it doesn't exist:

  $ modowners add '*' @ferris
  $ ls .github
  CODEOWNERS
  $ cat .github/CODEOWNERS
  * @ferris

It should add new patterns at the end by default

  $ modowners add Makefile @octocat
  $ cat .github/CODEOWNERS
  * @ferris
  Makefile @octocat

It should merges rules

  $ modowners add '*' @octocat
  $ cat .github/CODEOWNERS
  * @ferris @octocat
  Makefile @octocat

It should be idempotent:

  $ modowners add '*' @octocat
  $ cat .github/CODEOWNERS
  * @ferris @octocat
  Makefile @octocat

It should preserve comments

  $ echo >> .github/CODEOWNERS
  $ echo "# Rust files" >> .github/CODEOWNERS
  $ echo *.rs @rustacean >> .github/CODEOWNERS
  $ cat .github/CODEOWNERS
  * @ferris @octocat
  Makefile @octocat
  
  # Rust files
  *.rs @rustacean
  $ modowners add '*.rs' @ferris
  $ cat .github/CODEOWNERS
  * @ferris @octocat
  Makefile @octocat
  
  # Rust files
  *.rs @rustacean @ferris
