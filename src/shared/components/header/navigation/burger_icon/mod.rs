use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/shared/components/header/navigation/burger_icon/burger_icon.module.scss"
);

#[component]
pub fn BurgerIcon(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_toggle: Callback<()>,
) -> impl IntoView {
    let (rotation_deg, set_rotation_deg) = signal(90);

    let on_click = move |_| {
        on_toggle.run(());
        set_rotation_deg.update(|prev| *prev += 90);
    };

    view! {
        <button
            aria-expanded=move || is_open.get().to_string()
            aria-label="Menu"
            type="button"
            class=style::burger_icon
            style=move || format!("transform: rotate({}deg);", rotation_deg.get())
            on:click=on_click
        >
            {move || if is_open.get() { "X" } else { "|||" }}
        </button>
    }
}
