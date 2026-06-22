mod cli_struct;
mod enums_file;
mod subcommands;
mod functions;

use clap::Parser;
use cli_struct::{Cli};
use enums_file::{Commands};
use subcommands::{welcome, touch, cat, ls, pwd, cp, mv, mkdir};


fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Welcome { count } => { welcome(count); }
        Commands::Touch { name} => { touch(name); }
        Commands::Cat { name, amount } => { cat(name, amount);}
        Commands::Pwd {} => { pwd(); }
        Commands::Ls {} => { ls(); }
        Commands::Cp {source_name, dest_name} => { cp(source_name, dest_name); }
        Commands::Mv {source_name, dest_name} => { mv(source_name, dest_name);}
        Commands::Mkdir {name} => {mkdir(name);}
    }


}