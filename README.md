# Genereto

A simple static site generator to handle a blog website.

[<img src="https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg" width="300" align="center">](https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg)

With Genereto, you can:

* Write the template for your blog and index page.
* Write your articles in markdown.
* Generate your blog site by generating the html out of your markdown and applying the template.

Each article is divided in two sections: metadatas (written in yaml) and the article.

The metdata will be available in the template in the form of variables like `$GENERETO['title']`.

## Getting started

First, you need to create a project folder `genereto-project` (but can be called in any way) with a "content" and "
templates" folders.

```
genereto-project/
    content/
        /blog/2024-my-first-article/cover.png
        /blog/2024-my-first-article.md
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
# used in RSS
title: Blog
# used in RSS.
description: Description of the blog
# used in RSS or for templating purposes.
url: https://blog.fponzi.me
# Blog specific configuration
blog:
  # Optional custom title for the blog section. If not provided, the main title will be used.
  title: My Blog Title
```

Running genereto will create an output folder with the index, articles, and assets inside the `output_dirname` folder.
The `output_dirname` folder will be created inside the project-folder.

You can run genereto by running:

```shell
cargo run -- --project-path /home/user/blog/genereto-project
```

To generate a new project:

```shell
cargo run -- generate-project --project-path /path/to/project
```

By default, the project generation requires a git repository to prevent accidental overwrites. You can override this check using the `--override-git` flag:

```shell
cargo run -- generate-project --project-path /path/to/project --override-git
```

If you have draft articles, they will be built but will not be linked anywhere. If you want to hide them completely you
can use `--drafts hide`:

```shell
cargo run -- --project-path /home/user/blog/genereto-project --drafts hide
```

Genereto is not published as a compiled binary,
so for now you will need to fetch this repo and build it yourself by using rust and cargo.

## Box-Style Layout

The template system now supports a box-style layout for displaying content in a grid of boxes. This is particularly useful for creating link collections, resource lists, or article indexes. Each box can contain:

* Title (with optional link to external URL)
* Publication date
* Description
* Optional image

To use this layout:

1. Use the provided `main` template which includes the box-style CSS
2. In your content files, include the following metadata:
   * `title`: The title to display
   * `url`: (Optional) External URL to link to
   * `description`: Description text
   * `publish_date`: Publication date
   * `cover_image`: (Optional) Image to display

Example content file:

```yaml
title: Raft Made Simple
keywords: distributed systems, raft, consensus
publish_date: 2023-12-01
description: A great explanation of the Raft consensus algorithm with clear diagrams and examples
url: https://decentralizedthoughts.github.io/2020-12-12-raft-made-simple/
cover_image: https://example.com/raft.png
---

Content goes here...
```

The boxes will be styled with a clean, modern design including shadows and hover effects. The layout is responsive and will adjust to different screen sizes.

* `title` string: title of the article.
* `keywords` string: comma separated list of keywords.
* `publish_date` string: the published date of the article formatted as YYYY-mm-dd (e.g. 2023-01-01)
* `description` string: A small description.
* `is_draft` bool: Default false, if set to true it will skip processing this page.
* `show_table_of_contents` bool: Default false, if set to true it will add a ToC (if supported by the template)
* `cover_image` string: the cover image for this blog post. If empty the variable will use the value from the config file's `default_cover_image`.
* `url` string: Optional external URL for the article. If provided, the article title will link to this URL instead of the local page.

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
# if set to true, it will automatically add an H1 heading with the page title at the top of the content
add_title: true
```
### Inside articles
Inside the article, you can embed **todos**:

```
$GENERETO{TODO: need to rephrase this section}
```

The format matched is `$GENERETO{TODO`. When a TODO is present, your article will not be published even if is_draft is
false.

you can also write comments using the same syntax - the comments should be on the same line:

```
this is my article $GENERETO{give article a name!}
$GENERETO{TODO: I need to fix this chart.}
<img src="">
```

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
In the index.html, this content is replaced by the list of articles. For the index page, you can customize how each article is displayed by providing an HTML template between the start_content and end_content markers. For example:

```html
<!-- start_content -->
<div class="post">
    <h2><a href="$GENERETO['url']">$GENERETO['title']</a></h2>
    <div class="post-date">$GENERETO['publish_date']</div>
    <p class="post-description">$GENERETO['description']</p>
    <img src="$GENERETO['cover_image']" alt="$GENERETO['title']" class="post-image">
</div>
<!-- end_content -->
```

This template will be used for each article in the index, with the variables being replaced with the corresponding metadata from each article. If no template is provided (i.e., the content between start_content and end_content is empty), a default template will be used that displays the title, date, and description.

Inside the html templates, you have access to different variables; that take the form of `$GENERETO['variable_name']`:

* `publish_date`: as you defined it in your metadata section. It's YYYY-mm-dd
* `read_time_minutes`: estimated read time in minutes.
* `description`: as you defined it in your metadata section.
* `keywords`: as you defined it in your metadata section.
* `table_of_contents`: it's a simple `<ul><li>` based list generated from the headings. Each entry will have an id to
  quickly jump to the right heading.
* `last_modified_date`: format is like `2023-08-18`. It uses git to get the last modified date. If git is not present,
  it will use publish date instead.

### Feed RSS

Genereto will also generate a RSS feed for you, you should advertise it in your html (and somewhere in your website if
you want):

```
<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="rss.xml" />
```

### Development and iterating on articles

You can use the `--draft` argument. The supported options are:

* `build`: Default. Builds the draft page, but is not linked anywhere. Useful to share a draft.
* `dev`: Considers the draft page as a normal page. Useful during development to preview drafts.
* `hide`: Hides draft pages. They will not be built and will not be linked anywhere.

Adding a new git submodule:
```
git submodule add git@github.com:FedericoPonzi/genereto-template-main.git main
```
----

Genereto was presented in [this](https://blog.fponzi.me/2023-05-19-one-complex-setup.html) article.




