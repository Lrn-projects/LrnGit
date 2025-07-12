mod add;
pub mod branch;
mod commit;
mod config;
pub mod fs;
mod init;
mod log;
pub mod macros;
pub mod object;
pub mod parser;
pub mod refs;
mod status;
mod switch;
pub mod utils;
pub mod types;
mod push;
pub mod remote;
pub mod pack;
pub mod tcp;
pub mod pull;

use std::env;
use std::process::exit;

// Current version of lrngit
const VERSION: &str = "0.9.0";

pub fn lrngit_usage() -> &'static str {
    (r"
lrngit's cli.


Usage: lrngit command [options]


Commands:
    init            Init a local repository
    add             Add file to local repository
    commit          Commit to the local repository
    push            Push to remote repository
    pull            Pull from remote repositoy
    branch          Create a new branch or list all branches
    switch          Switch branch to the given one
    cat-file        Cat content of a given hash
    ls-file         Print content of the index file
    status          Show the status of the local repository
    log             Show the commit historic
    config          Manage config
    help            Show this help message
    version         Show the version

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of LrnGit
") as _
}


// enum of all lrngit command
#[derive(Debug, Clone)]
enum Commands {
    Init,
    Add { arg: String },
    Commit,
    Push,
    Pull,
    Branch,
    Switch,
    CatFile { arg: String },
    LsFile,
    Status,
    Remote,
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
                // Get index 2 because 0 is the binary, 1 the command and 2 the arg passed to the command
                .get(2)
                .unwrap_or_else(|| {
                    eprintln!("Please provide a file to add.");
                    exit(1);
                })
                .to_string(),
        },
        Some("commit") => Commands::Commit,
        Some("push") => Commands::Push,
        Some("pull") => Commands::Pull,
        Some("branch") => Commands::Branch,
        Some("switch") => Commands::Switch,
        Some("cat-file") => Commands::CatFile {
            arg: args
                // Get index 2 because 0 is the binary, 1 the command and 2 the arg passed to the command
                .get(2)
                .unwrap_or_else(|| {
                    eprintln!("Please provide a file to add.");
                    exit(1);
                })
                .to_string(),
        },
        Some("ls-file") => Commands::LsFile,
        Some("status") => Commands::Status,
        Some("remote") => Commands::Remote,
        Some("log") => Commands::Log,
        Some("config") => Commands::Config,
        Some("version") => Commands::Version,
        Some("help") => Commands::Help,
        _ => {
            lrncore::usage_exit::usage_and_exit("Invalid command", lrngit_usage());
            return;
        }
    };

    match command {
        Commands::Init => init::init_command(),
        Commands::Add { arg } => add::add_to_local_repo(arg),
        Commands::Commit => commit::commit_command(),
        Commands::Push => push::push_command(),
        Commands::Pull => pull::pull_command(),
        Commands::Branch => branch::branch_command(),
        Commands::Switch => switch::switch_command(),
        Commands::CatFile { arg } => object::utils::read_blob_file(&arg),
        Commands::LsFile => object::index::ls_file(),
        Commands::Status => status::status_command(),
        Commands::Remote => remote::remote_command(),
        Commands::Log => log::log_command(),
        Commands::Config => config::config_commands(),
        Commands::Version => lrncore::usage_exit::command_usage(&lrngit_version()),
        Commands::Help => lrncore::usage_exit::command_usage(lrngit_usage()),
    }
}

pub fn lrngit_version() -> String {
    let usage = format!("lrngit {VERSION}");
    usage
}
