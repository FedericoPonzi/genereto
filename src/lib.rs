pub(crate) use error::Result;
use std::path::PathBuf;
mod components;
pub(crate) mod error;
mod formats;

use formats::Config;

const COMPONENTS: &str = "components";
const TEMPLATES: &str = "templates";
const CONFIG_FILENAME: &str = "config.yml";

pub fn run(project: PathBuf) -> Result<()> {
    let config_path = project.join(CONFIG_FILENAME);
    let components = project.join(COMPONENTS);
    let config = Config::load_from_path(config_path)?;
    let template = project.join(TEMPLATES).join(config.template);
    build(components, template)
}

fn build(components: PathBuf, template: PathBuf) -> Result<()> {
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
