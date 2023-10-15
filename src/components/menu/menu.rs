use leptos::*;
use leptos::html::*;
use leptos_icons::BiIcon::{BiMenuRegular, BiSunRegular};
use leptos_icons::BsIcon::BsMoonStars;
use leptos_icons::IoIcon::IoCloseSharp;
use leptos_icons::Icon;

#[component]
pub fn LightDarkSwitch() -> impl IntoView {
    let (dark_mode_enabled, set_dark_mode_enabled) = create_signal(true);
    view! {
        {html().class("dark", move || dark_mode_enabled())}
        <div on:click=move |_| set_dark_mode_enabled.update(|value| {*value = !*value}) >
        <Show when=move || { !dark_mode_enabled() } fallback=|| view!{
            <Icon icon=Icon::from(BiSunRegular) class="inline" /> Enable light mode
        } >
            <Icon icon=Icon::from(BsMoonStars) class="inline"/> Enable dark mode
        </Show>
        </div>
    }
}

#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <input id="menu" class="hidden peer/menu" type="checkbox" />
        <label for="menu" class="peer-checked/menu:hidden z-50">
            <Icon icon=Icon::from(BiMenuRegular) class="ml-auto h-[1.5em] w-[1.5em] m-[0.25rem] md:h-[3rem] md:w-[3rem] md:m-[0.75rem] item-center" />
        </label>
        <label for="menu" class="hidden peer-checked/menu:block z-50">
            <Icon icon=Icon::from(IoCloseSharp) class="ml-auto h-[1.5em] w-[1.5em] m-[0.25rem] md:h-[3rem] md:w-[3rem] md:m-[0.75rem] item-center" />
        </label>
        <div class="w-full h-full fixed overflow-hidden max-w-0 transition-[max-width] duration-500 easy-in-out \
                    peer-checked/menu:max-w-full z-10 box-shadow-lg shadow-color-white animation-direction">
            <ul class="m-[80px] bg-slate-500">
                <li><LightDarkSwitch /></li>
                <li>menu2</li>
                <li>menu3</li>
                <li>menu4</li>
            </ul>
        </div>
    }
}