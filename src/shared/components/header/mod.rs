use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav>
                <A href="/">"logo"</A>
                <A href="/desserts">"დესერტები"</A>
                <A href="/about-us">"ჩვენი ისტორია"</A>
                <A href="/contact">"კონტაქტი"</A>
            </nav>

        </header>
    }
}
