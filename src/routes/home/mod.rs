mod components;
mod constants;

use crate::modules::landing::components::{hero::Hero, kitchen::Kitchen};
use crate::routes::home::components::desserts_list::DessertsList;
use leptos::prelude::*;

stylance::import_crate_style!(style, "src/routes/home/home_page.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <>
            <Hero />
            <DessertsList />
            <Kitchen />
        </>
    }
}
