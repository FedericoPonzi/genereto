use genereto::run;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
struct Opts {
    project_path: PathBuf,
}

fn main() {
    println!("Hello, world!");
    let env = env_logger::Env::new()
        .filter("GENERETO_LOG")
        .write_style("GENERETO_LOG_STYLE");
    env_logger::init_from_env(env);
    let opts = Opts::from_args();
    run(opts.project_path).expect("Error");
}
