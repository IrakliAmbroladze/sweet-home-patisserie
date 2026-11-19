use super::super::constants::DessertItem;
use crate::modules::desserts::components::dessert_item::DessertCard;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/modules/desserts/components/desserts_grid/desserts_grid.module.scss"
);

#[component]
pub fn DessertsGrid(items: &'static [DessertItem]) -> impl IntoView {
    view! {
        <div class=style::grid>
            {items.iter().map(|item| view! { <DessertCard item /> }).collect_view()}
        </div>
    }
}
