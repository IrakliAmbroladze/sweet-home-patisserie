use super::super::data;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/modules/contact/components/contact/contact.module.scss"
);
#[component]
pub fn Contact() -> impl IntoView {
    println!("{:?}", data::WEEKLY_SCHEDULE);
    view! { <>"I change this is contact view page"</> }
}
