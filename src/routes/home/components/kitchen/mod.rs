use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/routes/home/components/kitchen/kitchen.module.scss"
);
stylance::import_crate_style!(shared, "style/shared.module.scss");

#[component]
pub fn Kitchen() -> impl IntoView {
    view! {
        <section class=shared::container>
            <div class=style::wrapper>
                <div class=style::picture>
                    <img src="/images/kitchen.jpg" alt="picture" />
                </div>
                <div class=style::content>
                    <h2 class=style::title>
                        ჩვენი სამზარეულოდან
                    </h2>
                    <p class=style::description>
                        თითოეული დესერტი მზადდება სიყვარულით, საუკეთესო ინგრედიენტებით და პროფესიონალური ოსტატობით
                    </p>
                    <img src="/images/leaves.png" alt="leaves" class=style::leaves />
                    <img src="/images/plant.png" alt="plant" class=style::plant />

                </div>
            </div>
        </section>
    }
}
