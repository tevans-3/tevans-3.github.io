use std::fs;
use std::path::Path;

use tera::{Context, Tera};
use site::{chronosynclastic, courses, posts};

fn main() {
    let mut tera = Tera::default();
    let mut templates = Vec::new();
    for entry in fs::read_dir("templates").expect("failed to read templates dir") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("tera") {
            let name = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(".html")
                .unwrap_or(path.file_stem().unwrap().to_str().unwrap())
                .to_string();
            let content = fs::read_to_string(&path).unwrap();
            templates.push((name, content));
        }
    }
    let template_refs: Vec<(&str, &str)> = templates.iter().map(|(n, c)| (n.as_str(), c.as_str())).collect();
    tera.add_raw_templates(template_refs).expect("failed to load templates");
    let dist = Path::new("dist");

    if dist.exists() {
        fs::remove_dir_all(dist).expect("failed to clean dist");
    }

    // /
    write_page(&tera, "index", &Context::new(), dist.join("index.html"));

    // /posts
    let all_posts = posts::all_posts();
    let metas: Vec<_> = all_posts.iter().map(|p| p.meta()).collect();
    let mut ctx = Context::new();
    ctx.insert("posts", &metas);
    write_page(&tera, "posts", &ctx, dist.join("posts/index.html"));

    // /posts/<slug>
    for post in &all_posts {
        let mut ctx = Context::new();
        ctx.insert("title", &post.title);
        ctx.insert("content", &post.render());
        write_page(
            &tera,
            "post",
            &ctx,
            dist.join(format!("posts/{}/index.html", post.slug)),
        );
    }

    // /courses
    let years = courses::all_years();
    let rendered: String = years.iter().map(|y| y.render()).collect();
    let mut ctx = Context::new();
    ctx.insert("content", &rendered);
    write_page(&tera, "courses", &ctx, dist.join("courses/index.html"));

    // /projects
    write_page(
        &tera,
        "projects",
        &Context::new(),
        dist.join("projects/index.html"),
    );

    // /chronosynclastic-infundibula
    let mut ctx = Context::new();
    ctx.insert("reading_content", &chronosynclastic::reading_content());
    ctx.insert("quotes_content", &chronosynclastic::quotes_content());
    write_page(
        &tera,
        "chronosynclastic",
        &ctx,
        dist.join("chronosynclastic-infundibula/index.html"),
    );

    // /about
    write_page(
        &tera,
        "about",
        &Context::new(),
        dist.join("about/index.html"),
    );

    // copy static/
    copy_dir("static", &dist.join("static"));

    println!("Built site to dist/");
}

fn write_page(tera: &Tera, template: &str, ctx: &Context, path: impl AsRef<Path>) {
    let path = path.as_ref();
    let html = tera
        .render(template, ctx)
        .unwrap_or_else(|e| panic!("failed to render {}: {}", template, e));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create directories");
    }
    fs::write(path, html).expect("failed to write file");
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src = src.as_ref();
    let dst = dst.as_ref();
    fs::create_dir_all(dst).expect("failed to create static dir");
    for entry in fs::read_dir(src).expect("failed to read static dir") {
        let entry = entry.expect("failed to read entry");
        let dest_path = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(entry.path(), dest_path);
        } else {
            fs::copy(entry.path(), dest_path).expect("failed to copy file");
        }
    }
}
