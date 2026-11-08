use leptos::prelude::*;
use leptos_router::components::A;
stylance::import_crate_style!(style, "src/routes/home/components/hero/hero.module.scss");

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class=style::hero>
            <div class=style::container>
                <div class=style::wrapper>
                    <h2 class=style::title>"საოცარი გემო."</h2>
                    <h2 class=style::title>
                        "საკონდიტრო ხელოვნება."
                    </h2>
                    <div class=style::heart>
                        <img src="/images/heart.png" alt="" />
                    </div>
                    <div class=style::desserts_btn_wrapper>
                        <A href="/desserts" {..} class=style::desserts_btn>
                            "დაათვალიერე დესერტები >"
                        </A>
                    </div>
                </div>
            </div>
        </section>
    }
}
