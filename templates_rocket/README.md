# Using Templates for rendering HTML

In Rocket Framework, we can use templates to render HTML pages. We can use `tera` crate for this purpose. For example, we can create a `templates` directory in our project and put our HTML files in it. Then we can use `tera` to render these HTML files. But the html files should have `.tera` extension. For example, if we have `index.html` file, we should rename it to `index.html.tera`. Then we can use `tera` to render this file.

```rust
use rocket::response::content;
use tera::Tera;

#[get("/")]
pub fn index() -> content::Html<String> {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = tera::Context::new();
    let rendered = tera.render("index.html.tera", &context).unwrap();
    content::Html(rendered)
}
```

In this example, we are using `tera` to render `index.html.tera` file. We are using `Tera::new` to create a new instance of `Tera` and passing the path of our templates directory to it. Then we are creating a new `Context` and rendering `index.html.tera` file using `tera.render` method. Then we are returning the rendered HTML using `content::Html` type.


We can also pass some data to our HTML file using `tera`. For example, we can pass a `name` variable to our HTML file and use it in our HTML file. We can do this as follows:

```rust
use rocket::response::content;
use tera::Tera;

#[get("/")]
pub fn index() -> content::Html<String> {
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("name", "Aniket");
    let rendered = tera.render("index.html.tera", &context).unwrap();
    content::Html(rendered)
}
```

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Hello, {{ name }}</h1>
</body>
</html> 
```

In this example, we are passing a `name` variable to our HTML file and using it in our HTML file. We are using `tera.render` method to render our HTML file and passing the `context` to it. Then we are returning the rendered HTML using `content::Html` type.


We can also use to render css and javascript files using `tera`. For example, we can create a `style.css.tera` file and use `tera` to render it. We can do this as follows:

```rust
use rocket::response::content;
use tera::Tera;

#[get("/style.css")]
pub fn style() -> content::Css<String> {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = tera::Context::new();
    let rendered = tera.render("style.css.tera", &context).unwrap();
    content::Css(rendered)
}
```

In this example, we are using `tera` to render `style.css.tera` file. We are using `Tera::new` to create a new instance of `Tera` and passing the path of our templates directory to it. Then we are creating a new `Context` and rendering `style.css.tera` file using `tera.render` method. Then we are returning the rendered CSS using `content::Css` type.


For rendering javascript files, we can use `content::JavaScript` type. For example, we can create a `script.js.tera` file and use `tera` to render it. We can do this as follows:

```rust
use rocket::response::content;
use tera::Tera;

#[get("/script.js")]
pub fn script() -> content::JavaScript<String> {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = tera::Context::new();
    let rendered = tera.render("script.js.tera", &context).unwrap();
    content::JavaScript(rendered)
}
```

In this example, we are using `tera` to render `script.js.tera` file. We are using `Tera::new` to create a new instance of `Tera` and passing the path of our templates directory to it. Then we are creating a new `Context` and rendering `script.js.tera` file using `tera.render` method. Then we are returning the rendered javascript using `content::JavaScript` type.