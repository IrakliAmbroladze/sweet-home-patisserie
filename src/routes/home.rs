use crate::modules::{
    desserts::Desserts,
    landing::components::{hero::Hero, kitchen::Kitchen},
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <>
            <Hero />
            <Desserts />
            <Kitchen />
        </>
    }
}
