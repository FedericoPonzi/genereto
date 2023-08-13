# Genereto
A simple static site generator to handle a blog. 

Create a project folder "generto-project" with a "content" and "templates" folders.

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

The config file looks like this:

```
template: 'main' # select a template
output_dirname: 'output' # name for the folder that olds the generated files
```

Running genereto will create an output folder with the index, articles, and assets.

```shell
cargo run -- --project-path /home/user/blog/genereto-project
```

