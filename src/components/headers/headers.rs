use crate::components::logos::logo::Logo;
use crate::components::menu::menu::Menu;
use icondata::LuSearch;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_element_visibility;

#[island]
pub fn DynamicHeader(children: Children) -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let is_body_scrolling_under_header = RwSignal::new(false);
    Effect::new(move |_| {
        let visible = use_element_visibility(el);
        Effect::new(move |_| {
            is_body_scrolling_under_header.set(visible.get());
        });
    });
    view! {
        <div>
            <div class="h-1 top-0 absolute" node_ref=el />
            <header class=move || "flex flex-row top-0 px-5 z-50 sticky h-[80px]"
                    class=("dark:bg-black", move || !is_body_scrolling_under_header.get())
                    class=("bg-white", move || !is_body_scrolling_under_header.get()) >
                {children()}
            </header>
        </div>
    }
}

#[component]
pub fn BlogHeader() -> impl IntoView {
    view! {
        <DynamicHeader >
            <Logo />
            <Icon icon=LuSearch attr:class="text-white ml-auto items-center cursor-pointer h-[1.5rem] w-[1.5rem] my-[0.25rem] md:h-[3rem] md:w-[3rem] md:my-[0.75rem]"/>
            <Menu />
        </DynamicHeader>
    }
}
