mod actions;
mod app_context;
mod config;
use std::process::ExitCode;

use actions::Action;
use app_context::AppContext;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: CliAction,
}

#[derive(Debug, clap::Subcommand)]
enum CliAction {
    Init {
        #[clap()]
        name: String,
    },
}

fn main() -> ExitCode {
    let args = Args::parse();
    let ctx = match AppContext::default_init() {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };
    match args.action {
        CliAction::Init { name } => {
            match (actions::InitAction { project_name: name }).perform(&ctx) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    println!("Error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
    }
}
