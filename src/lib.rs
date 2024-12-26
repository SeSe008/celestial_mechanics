use leptos_router::path;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_meta::*;

// Modules
mod components;
mod pages;
mod utils;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Planet Properties Calculator"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes fallback=|| view!{ NotFound }>
                <Route path=path!("/") view=|| view!{ <Home/> }/>
                <Route path=path!("/*any") view=|| view!{ <NotFound/> }/>
            </Routes>
        </Router>
    }
}
