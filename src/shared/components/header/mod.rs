mod navigation;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::shared::components::header::navigation::Navigation;
stylance::import_crate_style!(style, "src/shared/components/header/header.module.scss");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class=style::wrapper>
            <div class=style::container>
                <div class=style::header_content>
                    <A href="/">
                        <div class=style::logo_wrapper>
                            <img src="/logo.png" alt="logo" />
                        </div>
                    </A>
                    <Navigation />
                </div>
            </div>

        </header>
    }
}
