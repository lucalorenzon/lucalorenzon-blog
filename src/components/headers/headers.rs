use crate::components::logos::logo::Logo;
use crate::components::menu::menu::Menu;
use icondata::LuSearch;
use leptos::html::Div;
use leptos::leptos_dom::is_browser;
use leptos::*;
use leptos_icons::Icon;
use leptos_use::use_element_visibility;

#[island]
pub fn DynamicHeader(children: Children) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let is_body_scrolling_under_header: MaybeSignal<bool> = if is_browser() {
        MaybeSignal::from(use_element_visibility(el))
    } else {
        MaybeSignal::from(false)
    };
    view! {
        <div class="h-1 top-0 absolute" node_ref=el />
        <header class=move || "flex flex-row top-0 px-5 z-50 sticky h-[80px]"
                class=("dark:bg-black", move || !is_body_scrolling_under_header())
                class=("bg-white", move || !is_body_scrolling_under_header()) >
            {children()}
        </header>
    }
}

#[component]
pub fn BlogHeader() -> impl IntoView {
    view! {
        <DynamicHeader >
            <Logo />
            <Icon icon=LuSearch class="text-white ml-auto items-center cursor-pointer h-[1.5rem] w-[1.5rem] my-[0.25rem] md:h-[3rem] md:w-[3rem] md:my-[0.75rem]"/>
            <Menu />
        </DynamicHeader>
    }
}
