use leptos::prelude::*;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts/desserts.module.scss"
);

#[component]
pub fn Desserts() -> impl IntoView {
    view! {
        <div class=style::grid>
            <div class=style::card>
                <div class=style::image_wrapper>
                    <img src="/images/dessert_1.jpg" alt="image" />
                </div>
                <div></div>
                <div></div>
            </div>
            <div class=style::card>
                <div class=style::image_wrapper>
                    <img src="/images/dessert_2.jpg" alt="image" />
                </div>
                <div></div>
                <div></div>
            </div>
            <div class=style::card>
                <div class=style::image_wrapper>
                    <img src="/images/dessert_3.jpg" alt="image" />
                </div>
                <div></div>
                <div></div>
            </div>
            <div class=style::card>
                <div class=style::image_wrapper>
                    <img src="/images/dessert_4.jpg" alt="image" />
                </div>
                <div></div>
                <div></div>
            </div>
        </div>
    }
}
