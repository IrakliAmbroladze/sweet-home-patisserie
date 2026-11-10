use super::super::constants::DESSERTS;
use leptos::prelude::*;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts/desserts.module.scss"
);

#[component]
pub fn Desserts() -> impl IntoView {
    view! {
        <div class=style::grid>
        {
           DESSERTS.iter().map(|item| {
                    view! {
                        <div class=style::card>
                            <div class=style::image_wrapper>
                                <img src=item.img alt="image" />
                            </div>
                            <h2 class=style::title>{item.title}</h2>
                            <div>{item.desc}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()
        }
        </div>
    }
}
