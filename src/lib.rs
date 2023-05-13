#[macro_use]
extern crate log;

pub(crate) use error::Result;
use std::path::PathBuf;
pub(crate) mod error;
use crate::components::{get_components, BaseComponent};
use crate::formats::MarkdownFile;
use formats::Config;
use std::fs;

mod components;
mod formats;

const COMPONENTS: &str = "components";
const TEMPLATES: &str = "templates";
const CONFIG_FILENAME: &str = "config.yml";

/// project: path to the project.
pub fn run(project: PathBuf) -> Result<()> {
    let config_path = project.join(CONFIG_FILENAME);
    let components = project.join(COMPONENTS);
    let config = Config::load_from_path(config_path)?;
    let template = project.join(TEMPLATES).join(&config.template);
    build(config, components, template)
}
fn build(config: Config, components_dir: PathBuf, template_dir: PathBuf) -> Result<()> {
    let components = get_components();
    for entry in fs::read_dir(components_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let comp_name = path.file_name().unwrap();
            let comp = components
                .get(comp_name.to_str().unwrap())
                .or_else(|| components.get("BaseComponent"))
                .unwrap();
            println!("Component name: {:?}", comp_name);
        } else {
            info!("Working on file: {:?}", path);
            let base_component = components.get("BaseComponent").unwrap();
            base_component.build(
                &config,
                std::fs::read_to_string(path).unwrap(),
                template_dir.as_path(),
            );
            (&entry);
        }
    }
    Ok(())
}

fn build_old(components: PathBuf, template: PathBuf) -> Result<()> {
    let load_view = template.join("index.html");
    let load_content = components.join("index.md");
    let mut view = std::fs::read_to_string(load_view)?;
    let markdown_content = std::fs::read_to_string(load_content)?;
    let html_content = load_markdown(&markdown_content);
    let start = view.find("<!-- start_content -->").unwrap();
    let end_pattern = "<!-- end_content -->";
    let end = view.find(end_pattern).unwrap();
    view.replace_range(start..end + end_pattern.len(), &html_content);
    println!("Result:\n {}", view);
    Ok(())
}

fn load_markdown(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
