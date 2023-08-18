# Genereto
A simple static site generator to handle a blog website. 

With Genereto, you can:
* write the template for your blog and index page. 
* Write your articles in markdown.
* Generate your blog site by generating the html out of your markdown and applying the template.

Each article is divided in metadata (written in yaml) and the article section. 
The metdata will be available in the template in the form of variables like $GENERETO['title'].

## Getting started

First you need to create a project folder "genereto-project" (can be called in any way) with a "content" and "templates" folders.
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

Create a config.yml file, it should look like like this:

```
template: 'main' # select a template
output_dirname: 'output' # name for the folder that olds the generated files
```

Running genereto will create an output folder with the index, articles, and assets inside the output_dirname folder.
The output_dirname folder will be created inside the project-folder.

You can run genereto by running:
```shell
cargo run -- --project-path /home/user/blog/genereto-project
```

## Writing articles
Every article is a markdown file divided in two sections: metadata and content. The two sections are divided by a number of "-".
As the name suggests, the metadata section contains metadata about the articles. It can be used to populate metadata headers and such.

This metadata section is in yaml. These are the supported fields:

* `title` string: title of the article.
* `keywords` string: comma separated list of keywords.
* `publish_date` string: the published date of the article
* `description` string: A small description.
* `is_draft` bool: Default false, if set to true it will skip processing this page.
* `show_table_of_contents` bool: Default false, if set to true it will add a ToC (if supported by the template)

## Templating
To create a template, you need two files:
* `index.html` will be used to list blog articles,
* `blog.html` is used for the single articles.

the section that is replaced with the html is:
```html
<!-- start_content -->
Hello world! This will be hidden in the result!
<!-- end_content -->
```
the content will be removed, so it can be used to test the template itself.

Inside the html templates, we have access to different variables; that take the form of `$GENERETO['variable_name']`:
* `publish_date`
* `read_time_minutes`
* `description`
* `keywords`
* `read_time_minutes`
* `table_of_contents`: it's a <ul><li> based list generated from the headings. Each entry will have an id 

----

Genereto was presented in [this](https://blog.fponzi.me/2023-05-19-one-complex-setup.html) article.