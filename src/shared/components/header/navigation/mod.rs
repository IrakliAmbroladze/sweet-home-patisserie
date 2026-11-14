mod burger_icon;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::shared::components::header::navigation::burger_icon::BurgerIcon;

stylance::import_crate_style!(
    style,
    "src/shared/components/header/navigation/navigation.module.scss"
);

#[component]
pub fn Navigation() -> impl IntoView {
    let (is_burger_menu_open, set_is_burger_menu_open) = signal(false);

    let toggle_menu = move |_| set_is_burger_menu_open.update(|prev| *prev = !*prev);
    let close_menu = move |_| set_is_burger_menu_open.set(false);

    view! {
        <>
            <BurgerIcon is_open=is_burger_menu_open on_toggle=toggle_menu />
            <nav class=move || {
                if is_burger_menu_open.get() {
                    format!("{} {}", style::navigation, style::open)
                } else {
                    style::navigation.to_string()
                }
            }>
                <A href="/desserts" on:click=close_menu.clone()>
                    "დესერტები"
                </A>
                <A href="/about-us" on:click=close_menu.clone()>
                    "ჩვენი ისტორია"
                </A>
                <A href="/contact" on:click=close_menu>
                    "კონტაქტი"
                </A>
            </nav>
        </>
    }
}
