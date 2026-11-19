use crate::modules::desserts::{components::desserts_grid::DessertsGrid, constants::DESSERTS_LIST};
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/modules/desserts/components/desserts/desserts.module.scss"
);
stylance::import_crate_style!(shared, "style/shared.module.scss");

#[component]
pub fn Desserts(#[prop(optional)] limit: Option<usize>) -> impl IntoView {
    let items = limit
        .and_then(|n| DESSERTS_LIST.get(..n))
        .unwrap_or(DESSERTS_LIST);
    view! {
        <section class=[shared::container, style::wrapper].join(" ")>
            <h2 class=style::title>"ჩვენი დესერტები"</h2>
            <DessertsGrid items />
        </section>
    }
}
