use crate::page_metadata::{PageMetadata, PageMetadataRaw};
use crate::DraftsOptions;
use anyhow::Context;
use regex::Regex;
use std::fs;
use std::path::Path;

pub(crate) const START_PATTERN: &str = "<!-- start_content -->";
pub(crate) const END_PATTERN: &str = "<!-- end_content -->";
pub fn load_compile_write(
    default_cover_image: &str,
    entry_path: &Path,
    drafts_options: &DraftsOptions,
    destination_path: &Path,
    template_raw: &str,
) -> anyhow::Result<Option<PageMetadata>> {
    let (content, metadata) = load_compile(default_cover_image, entry_path, template_raw)?;
    if metadata.is_draft && drafts_options.is_hide() {
        return Ok(None);
    }
    fs::write(destination_path, content).context("Failed writing to output page")?;
    Ok(Some(metadata))
}
pub fn load_compile(
    default_cover_image: &str,
    entry_path: &Path,
    template_raw: &str,
) -> anyhow::Result<(String, PageMetadata)> {
    let source_content = fs::read_to_string(entry_path)?;
    let (intermediate_content, metadata_raw) = compile_page_phase_1(&source_content)?;
    let (content, metadata) = compile_page_phase_2(
        intermediate_content,
        template_raw,
        metadata_raw,
        default_cover_image,
        entry_path,
    )?;
    Ok((content, metadata))
}

/// Compiles the html and applies the metadata substitutions
pub fn compile_page_phase_2(
    content: String,
    template_raw: &str,
    metadata_raw: PageMetadataRaw,
    default_cover_image: &str,
    entry_path: &Path,
) -> anyhow::Result<(String, PageMetadata)> {
    let metadata = PageMetadata::new(metadata_raw, &content, entry_path, default_cover_image);

    let mut final_page = template_raw.to_string();

    // If add_title is true, add an H1 with the page title at the top of the content
    let content_with_title = if metadata.add_title {
        format!("# {}\n\n{}", metadata.title, content)
    } else {
        content
    };

    let html_content = compile_markdown_to_html(&filter_out_comments(&content_with_title));
    let start = final_page.find(START_PATTERN).unwrap();
    let end = final_page.find(END_PATTERN).unwrap();

    final_page.replace_range(start..end + END_PATTERN.len(), &html_content);
    let final_page = metadata.apply(final_page);
    Ok((final_page, metadata))
}

// Split the source content into the metadata and the content
fn compile_page_phase_1(source_content: &str) -> anyhow::Result<(String, PageMetadataRaw)> {
    let pattern = Regex::new(r"---+\n").unwrap();
    let mut fields: Vec<&str> = pattern.splitn(source_content, 2).collect();
    if fields.len() < 2 {
        return Err(anyhow::anyhow!("Failed to find metadata in page"));
    }
    let (metadata, content) = (fields.remove(0), fields.remove(0));
    let content = add_ids_to_headings(content);

    let metadata: PageMetadataRaw = serde_yaml::from_str(metadata)
        .context("Failed to deserialize metadata, did you remember to put the metadata section?")?;
    Ok((content, metadata))
}

pub(crate) fn compile_markdown_to_html(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Parses the markdown to add ids to h1,h2,h3
/// to allow building the ToC.
pub(crate) fn add_ids_to_headings(content: &str) -> String {
    let mut content_new = String::new();
    let mut in_code_block = false;

    for line in content.lines() {
        let mut line_new = line.to_string();
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
        }
        if line.trim().starts_with('#') && !in_code_block {
            let title = line.trim_start().trim_start_matches('#').trim();
            let anchor = get_anchor_id_from_title(title);
            line_new = format!("{line}{{#{anchor}}}");
        }

        // this adds an extra \n on the last line, but it's ok.
        content_new.push_str(&format!("{line_new}\n"));
    }
    content_new
}

pub fn get_anchor_id_from_title(title: &str) -> String {
    remove_special_characters(title.trim())
        .replace(' ', "-")
        .to_lowercase()
}

fn remove_special_characters(input: &str) -> String {
    // Define a regular expression pattern to match special characters
    let pattern = Regex::new(r"[^a-zA-Z0-9\s]+").unwrap();

    // Replace matches with an empty string to remove special characters
    pattern.replace_all(input, "").to_string()
}

// search page_content for $GENERETO{comment } and filter it out.
// if the comment is in the same line of other text, it will remove only $GENERETO{}
// if the comment is in the only content of a line, it will remove the whole line
pub(crate) fn filter_out_comments(markdown_content: &str) -> String {
    let mut page_content_new = String::new();
    let mut in_code_block = false;

    for line in markdown_content.lines() {
        let mut line_new = line.to_string();
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
        }
        if in_code_block {
            page_content_new.push_str(&format!("{line_new}\n"));
            continue;
        }

        if line.trim().starts_with("$GENERETO{") {
            // full line comment, will remove the whole line.
            line_new = String::new();
        } else if line.contains("$GENERETO{") {
            // inline comment
            let comment_start = line.find("$GENERETO{").unwrap();
            let comment_end = line.find('}').unwrap();
            line_new = line.replace(&line[comment_start..=comment_end], "");
        }

        page_content_new.push_str(&format!("{line_new}\n"));
    }
    page_content_new
}

#[cfg(test)]
mod test {
    use crate::parser::filter_out_comments;

    #[test]
    fn test_filter_out_comments() {
        let page_content = "$GENERETO{comment}
        content
        some content$GENERETO{comment}
```
some code!!
```";
        let page_content_new = filter_out_comments(page_content);
        assert_eq!(
            page_content_new,
            "\n        content\n        some content\n```\nsome code!!\n```\n"
        );
    }

    #[test]
    fn test_add_ids_to_headings() {
        let page_content = r#"# heading1
        ## heading2
### hello world!
# Hello!
## How^! are you???? Hi"#;
        let expected = r"# heading1{#heading1}
        ## heading2{#heading2}
### hello world!{#hello-world}
# Hello!{#hello}
## How^! are you???? Hi{#how-are-you-hi}
";
        let page_content_new = super::add_ids_to_headings(page_content);
        assert_eq!(page_content_new, expected);
    }

    #[test]
    fn test_remove_special_characters() {
        let input = "Hello,..  world!!^-_.@dòł234disiduc";
        let expected = "Hello  worldd234disiduc";
        let output = super::remove_special_characters(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_anchor_id_from_title() {
        let input = "Hello world--..^^";
        let expected = "hello-world";
        let output = super::get_anchor_id_from_title(input);
        assert_eq!(output, expected);
    }
}
