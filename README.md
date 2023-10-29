# Genereto
A simple static site generator to handle a blog website. 

With Genereto, you can:
* Write the template for your blog and index page. 
* Write your articles in markdown.
* Generate your blog site by generating the html out of your markdown and applying the template.

Each article is divided in two sections: metadatas (written in yaml) and the article. 

The metdata will be available in the template in the form of variables like `$GENERETO['title']`.

## Getting started

First, you need to create a project folder `genereto-project` (but can be called in any way) with a "content" and "templates" folders.
```
genereto-project/
    content/
        /my-article/image.png
        my-article.md
    templates/
        main/
            res/
                logo.jpg
                style.css
            index.html
            blog.html
    config.yml
```
You can also use `sample-genereto-project` folder in this repository as a base example.

Create a `config.yml` file, it should look like this:

```
template: 'main' # select a template
output_dirname: 'output' # name for the folder that olds the generated files
```

Running genereto will create an output folder with the index, articles, and assets inside the `output_dirname` folder.
The `output_dirname` folder will be created inside the project-folder.

You can run genereto by running:
```shell
cargo run -- --project-path /home/user/blog/genereto-project
```
Because you can have draft articles, you can use `--skip-drafts` to not compile them:
```shell
cargo run -- --project-path /home/user/blog/genereto-project --skip-drafts
```
Genereto is not published as a compiled binary,
so for now you will need to fetch this repo and build it yourself by using rust and cargo.


## Writing articles
Every article is a markdown file divided in two sections: metadata and article content. The two sections are divided by a number of "-".

As the name suggests, the metadata section contains metadata about the article. It will be used in the template to populate metadata headers and such.

This metadata section is written in yaml. These are the supported fields:

* `title` string: title of the article.
* `keywords` string: comma separated list of keywords.
* `publish_date` string: the published date of the article in YYYY-mm-dd (e.g. 2023-01-01)
* `description` string: A small description.
* `is_draft` bool: Default false, if set to true it will skip processing this page.
* `show_table_of_contents` bool: Default false, if set to true it will add a ToC (if supported by the template)

As an example:
```yaml
title: My cool article
keywords: Hello world, article, cool
publish_date: 2023-01-01
description: This is an intro article to my blog.
# this is yaml, so comments work fine.
# draft = true means that we can skip compilation on the cli when using --skip-drafts
is_draft: true
# if set to true, it will add a `table_of_contents` variable in the template with the table of contents generated
# by this article.
show_table_of_contents: true
```

Inside the article, you can embed todos:
```
$GENERETO{TODO: need to rephrase this section}
```
The format matched is `$GENERETO{TODO`. When a TODO is present, your article will not be published even if is_draft is false.

## Templating
To create a template, you need two files:
* `index.html` the index page, which will be used to list blog articles,
* `blog.html` is the template used for single articles.

in both pages, the section that is replaced with the html is demarked by start_content and end_content.
```html
<!-- start_content -->
Hello world! This will be replaced, so you can use it in your template to see how the final resul will look like!
<!-- end_content -->
```
because the content will be removed, it can be used to test the template itself.

In the blog.html, this content is replaced with your article section.
In the index.html, this content is replaced by the list of articles.

Inside the html templates, you have access to different variables; that take the form of `$GENERETO['variable_name']`:
* `publish_date`: as you defined it in your metadata section. It's YYYY-mm-dd
* `read_time_minutes`: estimated read time in minutes. 
* `description`: as you defined it in your metadata section.
* `keywords`: as you defined it in your metadata section.
* `table_of_contents`: it's a simple `<ul><li>` based list generated from the headings. Each entry will have an id to quickly jump to the right heading. 
* `last_modified_date`: format is like `2023-08-18`. It uses git to get the last modified date. If git is not present, it will use publish date instead.

----

Genereto was presented in [this](https://blog.fponzi.me/2023-05-19-one-complex-setup.html) article.