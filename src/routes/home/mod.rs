use leptos::prelude::*;

stylance::import_crate_style!(style, "src/routes/home/home_page.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class=style::hero>
            <h1>"Sweet home patisseriSweet home patisse"</h1>
        </section>
    }
}
