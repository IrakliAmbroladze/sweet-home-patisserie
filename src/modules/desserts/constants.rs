#[derive(Debug)]
pub(super) struct DessertItem {
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
        desc: "კრუასანი და ფუნთუშები",
        img: "/images/dessert_3.jpg",
    },
    DessertItem {
        title: "დესერტი",
        desc: "ნამცხვრები და ტკბილეული",
        img: "/images/dessert_4.jpg",
    },
    DessertItem {
        title: "მაკარუნები",
        desc: "ფერადი ფრანგული ტკბილეული",
        img: "/images/dessert_5.jpg",
    },
    DessertItem {
        title: "ეკლერები",
        desc: "მოხარშული კრემის დესერტი",
        img: "/images/dessert_6.jpg",
    },
    DessertItem {
        title: "ფახლავა",
        desc: "ტრადიციული თხილეულით",
        img: "/images/dessert_7.jpg",
    },
    DessertItem {
        title: "ჩურჩხელა",
        desc: "ქართული ტრადიციული ტკბილეული",
        img: "/images/dessert_8.jpg",
    },
    DessertItem {
        title: "ფენოვანი",
        desc: "ხრაშუნა კარაქის ფენებით",
        img: "/images/dessert_9.jpg",
    },
    DessertItem {
        title: "ხილის ტარტები",
        desc: "სეზონური და ახალი ხილით",
        img: "/images/dessert_10.jpg",
    },
    DessertItem {
        title: "შოკოლადები",
        desc: "ხელნაკეთი შოკოლადის ნაკრები",
        img: "/images/dessert_11.jpg",
    },
    DessertItem {
        title: "ჯელატო / ნაყინი",
        desc: "ცივი და გამაგრილებებელი",
        img: "/images/dessert_12.jpg",
    },
];
