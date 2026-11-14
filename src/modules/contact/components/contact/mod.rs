use super::super::data;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/modules/contact/components/contact/contact.module.scss"
);
stylance::import_crate_style!(shared, "style/shared.module.scss");
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class=shared::container>
            <div class=style::wrapper>
                <h1 class=style::title>"მირაქი"</h1>
                <div class=style::subtitle>"დესერტები"</div>
                <div class=style::grid>
                    <div class=style::card>
                        <h2>"დაგვიკავშირდი"</h2>
                        <p>
                            <strong>"მისამართი:"</strong>
                            <br />
                            "მარიჯანის ქ. 16"
                            <br />
                            "0186 თბილისი, საქართველო"
                        </p>
                        <p>
                            <strong>"ტელეფონი:"</strong>
                            <br />
                            <a href="tel:+995598858635">"+995 598 85 86 35"</a>
                        </p>
                        <p>
                            <strong>"ბიზნესის ტიპი:"</strong>
                            <br />
                            "დესერტები და საკონდიტრო ნაწარმი"
                        </p>
                    </div>
                    <div class=style::card>
                        <h2>"სამუშაო საათები"</h2>
                        <ul class=style::schedule_list>
                            {data::WEEKLY_SCHEDULE
                                .into_iter()
                                .map(|(day, time)| {
                                    view! {
                                        <li>
                                            <span>{day}</span>
                                            <span>{time}</span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </div>
                    <div class=[style::card, style::delivery].join(" ")>
                        <h2>"მიტანის სერვისი"</h2>
                        <p>
                            "მიტანის სერვისი ხელმისაწვდომია კვირის ყველა დღეს, ჩვენი სტანდარტული სამუშაო საათების შესაბამისად."
                        </p>
                        <ul class=style::schedule_list>
                            {data::WEEKLY_SCHEDULE
                                .into_iter()
                                .map(|(day, time)| {
                                    view! {
                                        <li>
                                            <span>{day}</span>
                                            <span>{time}</span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </div>

                </div>
            </div>
        </div>
    }
}
