mod init;
pub mod utils;

use std::env;

// Current version of lrngit
// if modified and then running update command it will replace
// your current lrngit installation with the newer version
const VERSION: &'static str = "0.1.0";

#[derive(Debug, Clone)]
enum Commands {
    Init,
    Version,
    Help,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-v" => {
                lrncore::usage_exit::command_usage(&lrngit_version());
            }
            "--version" => {
                lrncore::usage_exit::command_usage(&lrngit_version());
            }
            _ => {}
        }
    }

    let command = match args.get(1).map(|s| s.as_str()) {
        Some("init") => Commands::Init,
        Some("version") => Commands::Version,
        Some("help") => Commands::Help,
        _ => {
            lrncore::usage_exit::usage_and_exit("Invalid command", &utils::lrngit_usage());
            return;
        }
    };

    match command {
        Commands::Init => init::init_local_repo(),
        Commands::Version => lrncore::usage_exit::command_usage(&lrngit_version()),
        Commands::Help => lrncore::usage_exit::command_usage(&utils::lrngit_usage()),
    }
}

pub fn lrngit_version() -> String {
    let usage = format!("lrngit {VERSION}");
    usage
}
