use crate::modules::desserts::components::dessert_item::DessertCard;

use super::super::constants::DESSERTS;
use leptos::prelude::*;
stylance::import_crate_style!(
    style,
    "src/modules/desserts/components/desserts_grid/desserts_grid.module.scss"
);

#[component]
pub fn DessertsGrid() -> impl IntoView {
    view! {
        <div class=style::grid>
            {DESSERTS.iter().map(|item| view! { <DessertCard item /> }).collect_view()}
        </div>
    }
}
