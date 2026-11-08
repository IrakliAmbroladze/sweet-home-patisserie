mod components;

use leptos::prelude::*;

use crate::routes::home::components::hero::Hero;

stylance::import_crate_style!(style, "src/routes/home/home_page.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <>
            <Hero />
        </>
    }
}
