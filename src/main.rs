use clap::{Parser, Subcommand, CommandFactory};
use env_logger::Env;
use genereto::run;
use genereto::DraftsOptions;
use log::info;
use std::path::PathBuf;

/// Genereto is a super simple static site generator.
#[derive(Parser)]
#[command(name = "genereto", version, about)]
struct Cli {
    /// Path to the generato-project folder. It can be named in any way, but it needs
    /// to contain a "content" folder, a "templates" folder and the config.yml file.
    #[arg(long)]
    project_path: Option<PathBuf>,
    
    /// How to handle draft pages
    #[arg(long, value_enum, default_value_t = DraftsOptions::Build)]
    drafts_options: DraftsOptions,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a new sample Genereto project
    GenerateProject {
        /// Path where to create the project
        #[arg(long)]
        project_path: PathBuf,
        /// Override git check - use with caution as it may cause irreversible overwrites
        #[arg(long)]
        override_git: bool,
    },
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::GenerateProject { project_path, override_git }) => {
            genereto::generate_project(&project_path, override_git).expect("Error");
            info!("Your project was successfully generated. Use `genereto --project-path {}` to run it.", project_path.display());
        }
        None => {
            let project_path = cli.project_path.unwrap_or_else(|| {
                eprintln!("error: --project-path is required when not using a subcommand\n");
                Cli::command().print_help().unwrap();
                std::process::exit(1);
            });
            
            let ret = run(project_path, cli.drafts_options).expect("Error");
            println!(
                "Website generation completed. Index path: {} ",
                ret.join("index.html").display()
            );
        }
    }
}


