mod cli_struct;
mod enums_file;
mod functions;

use clap::Parser;
use cli_struct::{Cli};
use enums_file::{Commands};
use functions::{welcome, touch, cat, ls, pwd};


fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Welcome { count } => { welcome(count); }
        Commands::Touch { name , extension} => { touch(name, extension); }
        Commands::Cat { name, amount } => { cat(name, amount);}
        Commands::Pwd {} => { pwd(); }
        Commands::Ls {} => { ls(); }
    }


}