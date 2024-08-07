pub mod blog;
pub mod blog_summary;
pub mod components;
pub mod docs;
pub mod docs_summary;
pub mod markdown_page;
pub mod pages_summary;
pub mod summary;

use axum::Router;
use dioxus::prelude::{ComponentFunction, Element, VirtualDom};
use std::{fs, net::SocketAddr, path::Path};
use tower_http::services::ServeDir;

pub mod routes {

    pub const SIGN_IN_UP: &str = "https://app.bionic-gpt.com";

    pub mod blog {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/blog/")]
        pub struct Index {}
    }

    pub mod marketing {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/")]
        pub struct Index {}
    }

    pub mod docs {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/docs/")]
        pub struct Index {}
    }

    pub mod pricing {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/pricing/")]
        pub struct Index {}
    }

    pub mod contact {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/contact/")]
        pub struct Index {}
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    fs::create_dir_all("dist").expect("Couldn't create dist folder");
    components::marketing::generate().await;
    summary::generate_docs(docs_summary::summary());
    summary::generate(blog_summary::summary());
    summary::generate_pages(pages_summary::summary()).await;
    summary::generate_blog_list(blog_summary::summary()).await;
    let src = Path::new("assets");
    let dst = Path::new("dist");
    summary::copy_folder(src, dst).expect("Couldn't copy folder");

    if std::env::var("DO_NOT_RUN_SERVER").is_err() {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

        // build our application with a route
        let app = Router::new().nest_service("/", ServeDir::new("dist"));

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        tracing::info!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

// Generic function to render a component and its props to a string
pub fn render_with_props<P: Clone + 'static, M: 'static>(
    root: impl ComponentFunction<P, M>,
    root_props: P,
) -> String {
    let mut vdom = VirtualDom::new_with_props(root, root_props);
    vdom.rebuild_in_place();
    let html = dioxus_ssr::render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

async fn render(ele: fn() -> Element) -> String {
    // create a VirtualDom with the app component
    let mut vdom = VirtualDom::new(ele);
    // rebuild the VirtualDom before rendering
    vdom.rebuild_in_place();

    // render the VirtualDom to HTML
    let html = dioxus_ssr::render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}
