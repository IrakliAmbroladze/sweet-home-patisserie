use leptos::prelude::*;
use leptos_router::components::A;
stylance::import_crate_style!(style, "src/shared/components/header/header.module.scss");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class=style::wrapper>
            <A href="/">"logo"</A>
            <nav class=style::navigation>
                <A href="/desserts">"დესერტები"</A>
                <A href="/about-us">"ჩვენი ისტორია"</A>
                <A href="/contact">"კონტაქტი"</A>
            </nav>

        </header>
    }
}
