use crate::routes::home::components::dessert_item::DessertCard;

use super::super::constants::DESSERTS;
use leptos::prelude::*;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts/desserts.module.scss"
);

#[component]
pub fn Desserts() -> impl IntoView {
    view! {
        <div class=style::grid>
            {DESSERTS.iter().map(|item| view! { <DessertCard item /> }).collect_view()}
        </div>
    }
}
