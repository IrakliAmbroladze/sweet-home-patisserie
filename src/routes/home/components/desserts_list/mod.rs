use leptos::prelude::*;

use crate::routes::home::components::desserts::Desserts;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts_list/desserts_list.module.scss"
);

#[component]
pub fn DessertsListh() -> impl IntoView {
    view! {
        <section class=style::wrapper>
            <h2 class=style::title>ჩვენი დესერტები</h2>
            <Desserts />
        </section>
    }
}
