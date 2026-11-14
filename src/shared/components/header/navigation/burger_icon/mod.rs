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
    let on_click = move |_| {
        on_toggle.run(());
    };

    view! {
        <button
            aria-expanded=move || is_open.get().to_string()
            aria-label="Menu"
            type="button"
            class=style::burger_icon
            on:click=on_click
        >
            {move || {
                if is_open.get() {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    }
                        .into_any()
                } else {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <line x1="3" y1="12" x2="21" y2="12"></line>
                            <line x1="3" y1="6" x2="21" y2="6"></line>
                            <line x1="3" y1="18" x2="21" y2="18"></line>
                        </svg>
                    }
                        .into_any()
                }
            }}
        </button>
    }
}
