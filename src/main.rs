use clap::{Parser, Subcommand};
use env_logger::Env;
use genereto::run;
use genereto::DraftsOptions;
use log::info;
use std::path::PathBuf;

/// Genereto is a super simple static site generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the generato-project folder. It can be named in any way, but it needs
    /// to contain a "content" folder, a "templates" folder and the config.yml file.
    #[arg(long)]
    project_path: Option<PathBuf>,
    #[arg(long)]
    #[clap(default_value_t, value_enum)]
    drafts_options: DraftsOptions,
    #[command(subcommand)]
    command: Option<Commands>,
}

// draft: build (default), dev, hide
#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates a new sample Genereto project.
    GenerateProject {
        #[arg(long)]
        project_path: PathBuf,
    },
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let args = Args::parse();
    if let Some(Commands::GenerateProject { project_path }) = args.command {
        genereto::generate_project(&project_path).expect("Error");
        info!("Your project was successfully generated. Use `genereto --project-path {}/genereto-project` to run it.", project_path.display());
        return;
    }
    let ret = run(
        args.project_path.expect("Project path not provided"),
        args.drafts_options,
    )
    .expect("Error");
    println!(
        "Website generation completed. Index path: {} ",
        ret.join("index.html").display()
    );
}
