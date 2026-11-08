use leptos::prelude::*;
use leptos_router::components::A;

stylance::import_crate_style!(
    style,
    "src/shared/components/header/navigation/navigation.module.scss"
);

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class=style::navigation>
            <A href="/desserts">"დესერტები"</A>
            <A href="/about-us">"ჩვენი ისტორია"</A>
            <A href="/contact">"კონტაქტი"</A>
        </nav>
    }
}
