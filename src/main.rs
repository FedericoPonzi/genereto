use genereto::run;
use std::path::PathBuf;
use env_logger::Env;
use clap::Parser;


/// Genereto is a super simple static site generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the generato-project folder. It can be named in any way, but it needs
    /// to contain a "content" folder, a "templates" folder and the config.yml file.
    #[arg(long)]
    project_path: PathBuf,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let args = Args::parse();
    run(args.project_path).expect("Error");
}
