use crate::page_metadata::get_anchor_id_from_title;

pub(crate) fn compile_markdown_to_html(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
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
            let title = line.trim_start_matches('#').trim();
            let anchor = get_anchor_id_from_title(title);
            line_new = format!("{line}{{#{anchor}}}");
        }

        content_new.push_str(&format!("{line_new}\n"));
    }
    content_new
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
            continue;
        }

        if line.trim().starts_with("$GENERETO{") {
            // full line comment, will remove the whole line.
            line_new = String::new();
        } else if line.contains("$GENERETO{") {
            // inline comment
            let comment_start = line.find("$GENERETO{").unwrap();
            let comment_end = line.find("}").unwrap();
            line_new = line.replace(&line[comment_start..=comment_end], "");
        } else {
            line_new += "\n"; // no comments.
        }
        page_content_new.push_str(&line_new);
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
        some content$GENERETO{comment}";
        let page_content_new = filter_out_comments(page_content);
        assert_eq!(page_content_new, "        content\n        some content");
    }
}
