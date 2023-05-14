#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use config::Config;
use std::{fs, path};
use regex::Regex;

mod config;

const CONFIG_FILENAME: &str = "config.yml";
// folder name for the markdown files with the content of the generated website
const CONTENT :&str = "content";
// Template folder name
const TEMPLATES: &str = "templates";

const START_PATTERN: &str = "<!-- start_content -->";
const END_PATTERN :&str = "<!-- end_content -->";

const OUTPUT_DIR: &str = "output";

/// project: path to the project.
pub fn run(project: PathBuf) -> anyhow::Result<()> {
    // todo: move to config.
    let config_path = project.join(CONFIG_FILENAME);
    let content = project.join(CONTENT);
    let config = Config::load_from_path(config_path)?;
    let template_dir = project.join(TEMPLATES).join(&config.template);
    let output_dir = project.join(OUTPUT_DIR);
    if output_dir.exists(){
        fs::remove_dir_all(&output_dir)?;
    }
    fs::create_dir_all(&output_dir)?;

    // TODO:
    if !content.exists() {
        //return error::IoError(std::io::Error::new(ErrorKind::NotFound, "Components folder not found, searched path: {components}"))
    }

    // iterate on all folders inside content
    for entry in fs::read_dir(&content)? {
        let path = entry?.path();
        let entry_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let template_file = template_dir.join(&format!("{entry_name}.html"));
        build(content.join(entry_name), template_file, output_dir.clone())?;
    }
    Ok(())
}

fn build(content_dir: PathBuf, template: PathBuf, output_dir: PathBuf) -> anyhow::Result<()> {
    debug!("Gonna build for {} with template {} and out_page {}", content_dir.display(), template.display(), output_dir.display());
    // iterate on all files in content folder, and call build_page
    for entry in fs::read_dir(content_dir)? {
        let entry_path = entry?.path();
        let filename = entry_path.file_name().unwrap().to_str().unwrap().to_string().replace(".md", ".html");

        // assume entry is a file (for now?)
        build_page(&template, entry_path, output_dir.join(filename))?;
    }
    Ok(())
}
fn build_page(template: &Path, in_page:PathBuf, out_page: PathBuf) -> anyhow::Result<()> {
    debug!("Gonna build page for {} with template {} and out_page {}", in_page.display(), template.display(), out_page.display());
    // 1 read in_page.
    let page = fs::read_to_string(in_page)?;
    let pattern = Regex::new(r"---+\n").unwrap();
    let mut fields: Vec<&str> = pattern.splitn(&page, 2).collect();
    let (metadata, content)  = (fields.remove(0), fields.remove(0));
    debug!("Metadata: {}", metadata);
    debug!("Content: {}",content);
    let mut template_view = fs::read_to_string(template)?;
    let html_content = load_markdown(content);
    let start = template_view.find(START_PATTERN).unwrap();
    let end = template_view.find(END_PATTERN).unwrap();

    template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

    //println!("Result:\n {}", template_view);
    fs::write(out_page, template_view)?;
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
