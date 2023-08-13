# Genereto
A simple static site generator to handle a blog. 

Create a project folder "genereto-project" (can be called in any way) with a "content" and "templates" folders.

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
You can also use sample-genereto-project.

The config.yml file looks like this:

```
template: 'main' # select a template
output_dirname: 'output' # name for the folder that olds the generated files
```

Running genereto will create an output folder with the index, articles, and assets inside the output_dirname folder.
The output_dirname folder will be created inside the project-folder.

```shell
cargo run -- --project-path /home/user/blog/genereto-project
```

## Writing articles
Every article is a markdown file divided in two sections: metadata and content. The two sections are divided by a number of "-".
As the name suggests, the metadata section contains metadata about the articles. It can be used to populate metadata headers and such.

This metadata section is used when building the article page. 

These are the supported options:

* `title` string: title of the article.
* `keywords` string: comma separated list of keywords.
* `publish_date` string: the published date of the article
* `description` string: A small description.

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
* publish_date
* read_time_minutes
* description
* keywords
* read_time_minutes

