use std::{env, process::exit};

pub struct GlobalConfig {
    pub user: GlobalConfigUser,
} 

pub struct GlobalConfigUser {
    pub name: String,
    pub email: String,
}

pub fn config_commands() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        exit(1);
    }
    match args[2].as_str() {
        "init" => init_global_config(),

        _ => {
            eprintln!("enter a config commands")
        }
    }
}

fn config_template() -> String {
    r"[user]
name = ''
email = ''
".to_string()
}

fn init_global_config() {
    let global_config_path: &str = "~/.lrngitconfig";
    
} 
