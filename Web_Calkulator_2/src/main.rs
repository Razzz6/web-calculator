use warp::Filter;
use tera::{Tera, Context};

#[tokio::main]
async fn main() {
    // Create Tera template engine and serve HTML files
    let tera = Tera::new("templates/**/*").unwrap();

    let route = warp::path::end()
        .and(warp::fs::dir("./static"))
        .or(warp::path::end().map(move || {
            let context = Context::new();
            warp::reply::html(tera.render("index.html", &context).unwrap())
        }));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
