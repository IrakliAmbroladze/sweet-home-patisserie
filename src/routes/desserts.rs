use leptos::prelude::*;

use crate::modules::desserts::Desserts;

#[component]
pub fn DessertsPage() -> impl IntoView {
    view! {
        <>
            <Desserts />
        </>
    }
}
