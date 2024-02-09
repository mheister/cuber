mod actions;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, clap::Subcommand)]
enum Action {
    Init {
        #[clap()]
        name: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Init { name } => {
            let _ = actions::InitAction { project_name: name };
        }
    }
}
