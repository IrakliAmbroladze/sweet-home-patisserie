pub struct DessertItem {
    pub title: &'static str,
    pub desc: &'static str,
    pub img: &'static str,
}

pub const DESSERTS: &[DessertItem] = &[
    DessertItem {
        title: "ტორტები",
        desc: "კლასიკური და საფირმო",
        img: "/images/dessert_1.jpg",
    },
    DessertItem {
        title: "ტკბილეული",
        desc: "მაფინები და მინი დესერტები",
        img: "/images/dessert_2.jpg",
    },
    DessertItem {
        title: "ცომეული",
        desc: "კრუასანი და ფუნტუშები",
        img: "/images/dessert_3.jpg",
    },
    DessertItem {
        title: "დესერტი",
        desc: "ნამცხვრები და ტკბილეული",
        img: "/images/dessert_4.jpg",
    },
];
