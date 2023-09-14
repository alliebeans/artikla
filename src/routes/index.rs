use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
    Template::render("base", context! { value: "" })
}

#[get("/login")]
pub fn login() -> Redirect {
    return Redirect::to("/articles");
}