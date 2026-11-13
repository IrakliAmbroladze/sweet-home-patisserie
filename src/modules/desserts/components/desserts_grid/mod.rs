use super::super::constants::DESSERTS;
use crate::modules::desserts::components::dessert_item::DessertCard;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

stylance::import_crate_style!(
    style,
    "src/modules/desserts/components/desserts_grid/desserts_grid.module.scss"
);

#[component]
pub fn DessertsGrid() -> impl IntoView {
    let location = use_location();

    let desserts = move || {
        if location.pathname.get() == "/desserts" {
            &DESSERTS
        } else {
            &DESSERTS[..4]
        }
    };

    view! {
        <div class=style::grid>
            {move || desserts().iter().map(|item| view! { <DessertCard item /> }).collect_view()}
        </div>
    }
}
