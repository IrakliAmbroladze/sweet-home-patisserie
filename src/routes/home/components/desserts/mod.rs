use leptos::prelude::*;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts/desserts.module.scss"
);

#[component]
pub fn Desserts() -> impl IntoView {
    view! {
        <div class=style::grid>
            <div class=style::card>1</div>
            <div class=style::card>2</div>
            <div class=style::card>3</div>
            <div class=style::card>4</div>
        </div>
    }
}
