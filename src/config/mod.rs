use std::{
    env,
    fs::File,
    io::{Read, Write},
    process::exit,
};

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
"
    .to_string()
}

fn init_global_config() {
    let config_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.lrngitconfig";
    println!("Initializing lrngit global configuration...");
    let mut config_file = match File::create(config_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create the config file: {}", e));
            exit(1);
        }
    };
    let template = config_template();
    let template_bytes = template.as_bytes();
    match config_file.write_all(template_bytes) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write the config file: {}", e));
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
    config_file.write_to_file(config_path).expect("Failed to update global config file");
}
