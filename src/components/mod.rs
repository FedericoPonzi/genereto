use crate::error::Result;
use crate::formats::{Config, MarkdownFile};
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::Path;

pub fn get_components() -> HashMap<String, Box<dyn Component>> {
    let components: Vec<Box<dyn Component>> = vec![Box::new(BaseComponent {})];
    components
        .into_iter()
        .map(|comp| (comp.get_name(), comp))
        .collect()
}

pub trait Component {
    // Builds the page
    fn build(&self, config: &Config, page: String, template_dir: &Path) -> Result<String>;

    // Unique component name
    fn get_name(&self) -> String;
}

/// Applied to all the top level .md files.
pub struct BaseComponent();
impl Component for BaseComponent {
    // TODO: Add filetype markdown or html. Assuming markdown
    // TODO: Add file name (so maybe metadatas)?
    fn build(&self, config: &Config, page: String, template: &Path) -> Result<String> {
        let html_content = Self::load_markdown(&page);
        let file_name = "index.html";
        let mut view = std::fs::read_to_string(template.join(file_name))?;
        let content_regex: Regex =
            Regex::new(r"<!-- start_content -->(.|\s)*<!-- end_content -->").unwrap();
        let res = content_regex.replace_all(&view, |caps: &Captures| html_content.clone());
        println!("Found: {:?}", res);

        //let start = view.find("<!-- start_content -->").unwrap();
        //let end_pattern = "<!-- end_content -->";
        //let end = view.find(end_pattern).unwrap();
        //view.replace_range(start..end + end_pattern.len(), &html_content);
        //println!("Result:\n {}", view);
        Ok("".to_string())
    }

    fn get_name(&self) -> String {
        "BaseComponent".to_string()
    }
}
impl BaseComponent {
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
}
