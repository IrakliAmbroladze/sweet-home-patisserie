use leptos::prelude::*;
stylance::import_crate_style!(
    style,
    "src/routes/home/components/desserts/desserts.module.scss"
);

#[component]
pub fn Desserts() -> impl IntoView {
    view! {
        <div class=style::grid>
            <div>1</div>
            <div>2</div>
            <div>3</div>
            <div>4</div>
        </div>
    }
}
