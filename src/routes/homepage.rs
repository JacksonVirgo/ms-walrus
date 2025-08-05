use axum::response::Html;
use maud::{Markup, html};

use crate::builders::webpage::WebPageBuilder;

pub async fn index() -> Html<String> {
    let page = WebPageBuilder::new().body(html! {
        main."flex flex-col gap-2 p-8" {
            h1."text-2xl font-bold" { "Ms. Walrus" }
            h2."text-lg" {"Single, proud, and tusks ready for some music."}
            div."py-2" {
                a href="/host" class="border border-black p-2 rounded-md hover:cursor-pointer" {
                    "Host Game"
                }
            }
        }
    });
    Html(page.build().into_string())
}

pub async fn clicked() -> Html<String> {
    let markup: Markup = html! {
        p { "You clicked the button!" }
    };
    Html(markup.into_string())
}
