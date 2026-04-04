mod blog;
mod chronosynclastic;
mod course;
mod courses;
mod posts;

use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/posts")]
fn posts_index() -> Template {
    let posts: Vec<_> = posts::all_posts().iter().map(|p| p.meta()).collect();
    Template::render("posts", context! { posts })
}

#[get("/posts/<slug>")]
fn post_page(slug: &str) -> Option<Template> {
    let all = posts::all_posts();
    let post = all.iter().find(|p| p.slug == slug)?;
    let rendered = post.render();
    Some(Template::render(
        "post",
        context! { title: &post.title, content: rendered },
    ))
}

#[get("/courses")]
fn courses_index() -> Template {
    let years = courses::all_years();
    let rendered: String = years.iter().map(|y| y.render()).collect();
    Template::render("courses", context! { content: rendered })
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", context! {})
}

#[get("/chronosynclastic-infundibula")]
fn chronosynclastic_infundibula() -> Template {
    let reading = chronosynclastic::reading_content();
    let quotes = chronosynclastic::quotes_content();
    Template::render(
        "chronosynclastic",
        context! { reading_content: reading, quotes_content: quotes },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, posts_index, post_page, courses_index, projects, chronosynclastic_infundibula, about],
        )
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
