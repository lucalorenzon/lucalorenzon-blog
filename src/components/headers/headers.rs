use leptos::*;
use leptos::html::{Div};
use leptos::leptos_dom::{is_browser};
use leptos_icons::Icon;
use leptos_icons::LuIcon::LuSearch;
use leptos_use::use_element_visibility;
use crate::components::logos::logo::Logo;
use crate::components::menu::menu::Menu;

#[component]
pub fn BlogHeader( ) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let is_body_scrolling_under_header: MaybeSignal<bool> = if is_browser() {
        MaybeSignal::from(use_element_visibility(el))
    } else {
        MaybeSignal::from(false)
    };
    view! {
        <div class="h-1 top-0 absolute" node_ref=el />
        <header class="flex flex-row top-0 px-5 sticky z-10"
                class=("dark:bg-black", move || !is_body_scrolling_under_header())
                class=("bg-white", move || !is_body_scrolling_under_header())>
            <Logo />
            <Icon icon=Icon::from(LuSearch) class="ml-auto items-center cursor-pointer h-[1.5rem] w-[1.5rem] my-[0.25rem] md:h-[3rem] md:w-[3rem] md:my-[0.75rem]"/>
            <Menu />
        </header>
    }
}