use leptos::prelude::*;

use super::super::data;

stylance::import_crate_style!(
    style,
    "src/modules/about_us/components/about_us/about_us.module.scss"
);
stylance::import_crate_style!(shared, "style/shared.module.scss");

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
        <div class=shared::container>
            <div class=style::wrapper>
                <section class=style::header>
                    <h1>"ჩვენ შესახებ"</h1>
                    <div class=style::subtitle>
                        "სიზუსტე და სიტკბოს ხელოვნება"
                    </div>
                </section>

                <section class=style::story_banner>
                    <h2>
                        "სადაც ყველა დესერტი საკუთარ ისტორიას გვიყვება"
                    </h2>
                    <p>
                        "უკვე ორი წელია, რაც „მირაქი“ ერთი მარტივი რწმენით იქმნება: ცხობა ხელოვნებაცაა და ადამიანების ერთმანეთთან დაახლოების გზაც. ახალგაზრდა კონდიტერის სიყვარულითა და თითოეული ეტაპისადმი განსაკუთრებული ყურადღებით, საუკეთესო ინგრედიენტებს დაუვიწყარ ტკბილეულად ვაქცევთ. გვიყვარს არა მხოლოდ ჩვენი ნამუშევრების გაზიარება, არამედ იმ ხელობისა და პროცესის გაცნობაც, რომელიც თითოეული დესერტის მიღმა დგას."
                    </p>
                </section>

                <section class=style::pillars_grid>
                    {data::PILLARS
                        .into_iter()
                        .map(|(badge, title, description)| {
                            view! {
                                <div class=style::card>
                                    <span class=style::badge>{badge}</span>
                                    <h3>{title}</h3>
                                    <p>{description}</p>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </section>
            </div>
        </div>
    }
}
