use clap::{Parser};


#[derive(Parser)]
#[command(name = "crates")]
#[command(about = "Delete the .cargo/registry folder", long_about = None)]
pub struct Cli {
    
    #[arg(long, short)]
    pub clean: bool,
}


impl Cli {
   
    pub fn get_command(&self) -> Option<CommandType> {
        
        if self.clean {
            Some(CommandType::Clean)
        }else {
            None
        }
    }
}

pub enum CommandType {
    Clean,
}