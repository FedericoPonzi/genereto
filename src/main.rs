use clap::{CommandFactory, Parser, Subcommand};
use env_logger::Env;
use genereto::run;
use genereto::verify::{self, Check};
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

    /// Run verification checks after building. Optionally specify which checks to run.
    #[arg(long = "verify", value_enum, num_args = 0..)]
    verify_checks: Option<Vec<Check>>,

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
    /// Run verification checks on a built project
    Verify {
        /// Path to the genereto project folder
        #[arg(long)]
        project_path: PathBuf,
        /// Which checks to run (all if omitted)
        #[arg(value_enum)]
        checks: Vec<Check>,
    },
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::GenerateProject {
            project_path,
            override_git,
        }) => {
            genereto::generate_project(&project_path, override_git).expect("Error");
            info!("Your project was successfully generated. Use `genereto --project-path {}` to run it.", project_path.display());
        }
        Some(Commands::Verify {
            project_path,
            checks,
        }) => {
            let config = genereto::GeneretoConfig::load_from_folder(project_path)
                .expect("Failed to load project config");
            let output_dir = config.output_dir_path.clone();
            let issues = verify::run_checks(&config, &checks, &output_dir);
            let count = verify::report_issues(&issues);
            if count > 0 {
                eprintln!("\n{} verification issue(s) found.", count);
                std::process::exit(1);
            } else {
                println!("All verification checks passed.");
            }
        }
        None => {
            let project_path = cli.project_path.unwrap_or_else(|| {
                eprintln!("error: --project-path is required when not using a subcommand\n");
                Cli::command().print_help().unwrap();
                std::process::exit(1);
            });

            let output_dir = run(project_path.clone(), cli.drafts_options).expect("Error");
            println!(
                "Website generation completed. Index path: {} ",
                output_dir.join("index.html").display()
            );

            // Run verify checks if --verify was passed
            if let Some(checks) = cli.verify_checks {
                let config = genereto::GeneretoConfig::load_from_folder(project_path)
                    .expect("Failed to load project config");
                let issues = verify::run_checks(&config, &checks, &output_dir);
                let count = verify::report_issues(&issues);
                if count > 0 {
                    eprintln!("\n{} verification issue(s) found.", count);
                    std::process::exit(1);
                } else {
                    println!("All verification checks passed.");
                }
            }
        }
    }
}
