use crate::modules::desserts::constants;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/modules/desserts/components/dessert_item/dessert_item.module.scss"
);

#[component]
pub fn DessertCard(item: &'static constants::DessertItem) -> impl IntoView {
    view! {
        <div class=style::card>
            <div class=style::image_wrapper>
                <img src=item.img alt=item.title />
            </div>
            <h2 class=style::title>{item.title}</h2>
            <p>{item.desc}</p>
        </div>
    }
}
