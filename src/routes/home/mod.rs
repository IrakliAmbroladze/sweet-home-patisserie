use crate::modules::desserts::components::desserts_list::DessertsList;
use crate::modules::landing::components::{hero::Hero, kitchen::Kitchen};
use leptos::prelude::*;

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
