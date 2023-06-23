## Writing articles
Every article is a markdown file divided in two sections: metadata and content. The two sections are divided by a number of "-".
As the name suggests, the metadata section contains metadata about the articles.

This metadata is used when building the article page. These are the supported options:

* `title` string
* `keywords` string: comma separated list of keywords.
* `publish_date` string
* `description` string

## Templating
To create a template, you just need two files:
* index.html will be used to list blog articles,
* blog.html is used for the single articles.

the section that is replaced with the html is:
```html
<!-- start_content -->
Hello world! This will be hidden in the result!
<!-- end_content -->
```
the content will be removed, so it can be used to test the template itself.

Inside of the html templates, we have access to different variables, that take the form of `$GENERETO['variable_name']`:
* publish_date
* read_time_minutes
* description
* keywords
* read_time_minutes

