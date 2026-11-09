use leptos::prelude::*;

use crate::routes::home::components::desserts::Desserts;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts_list/desserts_list.module.scss"
);
stylance::import_crate_style!(shared, "style/shared.module.scss");

#[component]
pub fn DessertsList() -> impl IntoView {
    view! {
        <section class=[shared::container, style::wrapper].join(" ")>
            <h2 class=style::title>ჩვენი დესერტები</h2>
            <Desserts />
        </section>
    }
}
