use leptos::prelude::*;
stylance::import_crate_style!(style, "src/shared/components/button/button.module.scss");

#[component]
pub fn Button() -> impl IntoView {
    view! { <button class=style::wrapper>"Test button"</button> }
}
