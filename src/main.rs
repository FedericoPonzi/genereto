use genereto::run;
use std::path::PathBuf;
use env_logger::Env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
struct Opts {
    project_path: PathBuf,
}

fn main() {
    println!("Hello, world!");
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let opts = Opts::from_args();
    run(opts.project_path).expect("Error");
}
