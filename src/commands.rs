use crate::{console, file_ops, progress};

use std::path::{PathBuf};
use std::env;
use std::io::{self, Write};
use colored::*; 
use std::process;



pub fn clean_registry(){
    console::clear_console();
    
    let cargo_registry = match get_cargo_registry_path(){
        Some(path) => path,
        None => return,
    };


    if !cargo_registry.exists(){
        println!("{}", "The file does not exist.".red());
        return;
    }

    println!("{}", format!("Warning! The folder {:?} will be deleted.", cargo_registry).yellow());

    if !prompt_confirmation(){
        println!("{}", "Operation canceled.".red());
        process::exit(0);
    }


    let items = file_ops::collect_paths(&cargo_registry);

    let pb = progress::setup_progress_bar(items.len());

    for path in items {
        file_ops::delete_path(&path);
        pb.inc(1);
    }

    file_ops::delete_dir(&cargo_registry);

    pb.finish_with_message("Deletion complete!".green().to_string());
}








// private function 


fn get_cargo_registry_path() -> Option<PathBuf> {
    if cfg!(windows) {
        let home_dir = env::var("USERPROFILE").expect("User folder could not be found");
        Some(PathBuf::from(format!("{}\\.cargo\\registry", home_dir)))
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        let home_dir = env::var("HOME").expect("User folder could not be found");
        Some(PathBuf::from(format!("{}/.cargo/registry", home_dir)))
    } else {
        // Si l'OS n'est pas supporté, on affiche un message d'erreur
        println!("{}", "Unsupported OS".red());
        None
    }
}

fn prompt_confirmation() -> bool {
    loop {
        println!("{}", "Do you want to continue? (y/n)".cyan());

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();

        match input.as_str(){
            "y" => return true,  
            "n" => return false,
            _ => {
                println!("{}", "Invalid input. Please enter 'y' for yes or 'n' for no.".red());
            }
        }
    }
}