mod add;
mod commit;
mod init;
pub mod utils;
mod config;
pub mod branch;
mod log;
mod status;
pub mod macros;

use std::env;
use std::process::exit;


// Current version of lrngit
// if modified and then running update command it will replace
// your current lrngit installation with the newer version
const VERSION: &str = "0.1.0";

// enum of all lrngit command
#[derive(Debug, Clone)]
enum Commands {
    Init,
    Add { arg: String },
    Commit,
    CatFile{ arg: String },
    LsFile,
    Status,
    Log,
    Config,
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
        Some("add") => Commands::Add {
            arg: args
                // get index 2 because 0 is the binary, 1 the command and 2 the arg passed to the command
                .get(2)
                .unwrap_or_else(|| {
                    eprintln!("Please provide a file to add.");
                    exit(1);
                })
                .to_string(),
        },
        Some("commit") => Commands::Commit,
        Some("cat-file") => Commands::CatFile {
            arg: args
                // get index 2 because 0 is the binary, 1 the command and 2 the arg passed to the command
                .get(2)
                .unwrap_or_else(|| {
                    eprintln!("Please provide a file to add.");
                    exit(1);
                })
            .to_string(),
        },
        Some("ls-file") => Commands::LsFile,
        Some("status") => Commands::Status,
        Some("log") => Commands::Log,
        Some("config") => Commands::Config, 
        Some("version") => Commands::Version,
        Some("help") => Commands::Help,
        _ => {
            lrncore::usage_exit::usage_and_exit("Invalid command", utils::lrngit_usage());
            return;
        }
    };

    match command {
        Commands::Init => init::init_local_repo(),
        Commands::Add { arg } => add::add_to_local_repo(arg),
        Commands::Commit =>  commit::commit_command(),
        Commands::CatFile { arg } => utils::read_blob_file(&arg), 
        Commands::LsFile => utils::ls_file(),
        Commands::Status => status::status_command(),
        Commands::Log => log::log_command(),
        Commands::Config => config::config_commands(),
        Commands::Version => lrncore::usage_exit::command_usage(&lrngit_version()),
        Commands::Help => lrncore::usage_exit::command_usage(utils::lrngit_usage()),
    }
}

pub fn lrngit_version() -> String {
    let usage = format!("lrngit {VERSION}");
    usage
}
