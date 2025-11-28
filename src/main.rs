use clap::Parser;
use std::env;
use colored::*; 


mod cli;
mod console;
mod file_ops;
mod progress;
mod commands;



use cli::{Cli, CommandType};



fn main()  {
    let _cli = Cli::parse();

    match _cli.get_command(){

        Some(CommandType::Clean) => {
    
            commands::clean_registry();
        }

        None => {
            // No command, displaying a standard message
            println!("{}", format!("Usage: {} --clean", env::args().next().unwrap()).red());
        }
    }

}
