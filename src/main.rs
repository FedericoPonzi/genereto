use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::File;
use crate::formats::Config;
use genereto::build;

mod formats;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
struct Opts{
    project_path: PathBuf
}

fn main() {
    println!("Hello, world!");
    let env = env_logger::Env::new()
        .filter("GENERETO_LOG")
        .write_style("GENERETO_LOG_STYLE");
    env_logger::init_from_env(env);

    let opts = Opts::from_args();
    let project_path = opts.project_path;
    let config_path = project_path.join("config.yml");
    let config: Config = serde_yaml::from_reader(&File::open(config_path).unwrap()).unwrap();
    let components = project_path.join("components");
    let template = project_path.join("template").join(config.template);

    build(components, template).expect("Error");
}
