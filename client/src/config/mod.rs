use std::{
    env,
    fs::File,
    io::Write,
    process::exit,
};

use lrncore::path::get_current_path;
use serde::Serialize;

pub struct GlobalConfig {
    pub user: GlobalConfigUser,
    pub url: GlobalConfigUrl,
}

pub struct GlobalConfigUser {
    pub name: String,
    pub email: String,
}

pub struct GlobalConfigUrl {
    pub remote: String,
}

#[derive(Serialize, Debug)]
pub struct LocalConfig {
    pub remotes: Remotes,
}

#[derive(Serialize, Debug)]
pub struct Remotes {
    pub url: String,
    #[allow(dead_code)]
    pub fetch: String,
}

pub fn config_commands() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        exit(1);
    }
    match args[2].as_str() {
        "init" => init_global_config(),
        "cat" => cat_global_config(),

        _ => {
            eprintln!("enter a config commands")
        }
    }
}

fn config_template() -> String {
    r"[user]
name = ''
email = ''

[url]
remote = ''
"
    .to_string()
}

fn init_global_config() {
    let config_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.lrngitconfig";
    println!("Initializing lrngit global configuration...");
    let mut config_file = match File::create(config_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create the config file: {e}"));
            exit(1);
        }
    };
    let template = config_template();
    let template_bytes = template.as_bytes();
    match config_file.write_all(template_bytes) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write the config file: {e}"));
            exit(1);
        }
    };
    write_global_config();
}

fn write_global_config() {
    let config_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.lrngitconfig";
    let mut config_file =
        ini::Ini::load_from_file(&config_path).expect("Failed to open global config file");
    let mut user = config_file.with_section(Some("user"));
    // username
    let mut user_name_stdi = String::new();
    print!("Enter a new name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_name_stdi).unwrap();
    user.set("name", user_name_stdi.trim_end());
    // email
    print!("Enter a new email: ");
    let mut user_email_stdi = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_email_stdi).unwrap();
    user.set("email", user_email_stdi.trim_end());
    config_file
        .write_to_file(config_path)
        .expect("Failed to update global config file");
}

/// Parse the global configuration file in the home directory
pub fn parse_global_config() -> GlobalConfig {
    let config_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.lrngitconfig";
    let ini_file =
        ini::Ini::load_from_file(&config_path).expect("Failed to open global config file");
    let user_section = ini_file
        .section(Some("user"))
        .expect("Missing [user] section in config file");
    let url_section = ini_file
        .section(Some("url"))
        .expect("Missing [url] section in config file");
    let name = user_section
        .get("name")
        .expect("Missing 'name' in [user] section")
        .to_string();
    let email = user_section
        .get("email")
        .expect("Missing 'email' in [user] section")
        .to_string();
    let remote = url_section
        .get("remote")
        .expect("Missing 'remote' in [url] section")
        .to_string();
    GlobalConfig {
        user: GlobalConfigUser { name, email },
        url: GlobalConfigUrl { remote },
    }
}

/// Parse the configuration file in local repository
pub fn parse_local_config() -> LocalConfig {
    let config_path = get_current_path() + "/.lrngit/config";
    let ini_file =
        ini::Ini::load_from_file(&config_path).expect("Failed to load local config file");
    let remote_section = ini_file
        .section(Some("remote"))
        .expect("Missing [remote] section in local config file");
    let url = remote_section
        .get("url")
        .expect("Missing 'url' in [remote] section")
        .to_owned();
    let fetch = remote_section
        .get("fetch")
        .expect("Missing 'fetch' in [remote] section")
        .to_owned();
    LocalConfig {
        remotes: Remotes { url, fetch },
    }
}

pub fn update_remote_url_local_config(url: &str) {
    let config_path = get_current_path() + "/.lrngit/config";
    let mut config_file =
        ini::Ini::load_from_file(&config_path).expect("Failed to open global config file");
    let mut remote = config_file.with_section(Some("remote"));
    remote.set("url", url);
    config_file
        .write_to_file(config_path)
        .expect("Failed to update global config file");
}

fn cat_global_config() {
    let config = parse_global_config();
    println!(
        "[user]\nname = {}\nemail = {}\n\n[url]\nremote = {}",
        config.user.name, config.user.email, config.url.remote
    );
}

/// Create the config file for the local repository using a basic template
pub fn init_config_repo() {
    let mut config =
        File::create_new(".lrngit/config").expect("Failed to create local repository config file");
    let template = r"[remote]
url = ''
fetch = +refs/heads/*:refs/remotes/origin/*
"
    .to_string();
    config
        .write_all(template.as_bytes())
        .expect("Failed to write template in config file");
}
