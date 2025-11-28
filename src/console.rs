
use std::process::Command;
use std::env;

pub fn clear_console(){
    if env::consts::OS != "windows" {
        let _ = Command::new("clear").output();
        return;
    }
    let _ = Command::new("cmd").arg("/C").arg("cls").output();
}
