use icondata::{BiMenuRegular, BiSunRegular, BsMoonStars, IoCloseSharp};
use leptos::prelude::*;
use leptos_icons::Icon;

#[island]
pub fn LightDarkSwitch() -> impl IntoView {
    let (dark_mode_enabled, set_dark_mode_enabled) = signal(true);
    Effect::new(move |_| {
        let enabled = dark_mode_enabled.get();
        if let Some(el) = document().document_element() {
            let _ = el.class_list().toggle_with_force("dark", enabled);
        }
    });
    view! {
        <div on:click=move |_| set_dark_mode_enabled.update(|value| {*value = !*value}) >
        <Show when=move || { !dark_mode_enabled.get() } fallback=|| view!{
            <Icon icon=BiSunRegular attr:class="inline" /> Enable light mode
        } >
            <Icon icon=BsMoonStars attr:class="inline"/> Enable dark mode
        </Show>
        </div>
    }
}

#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <input id="menu" class="hidden peer/menu" type="checkbox" />
        <label for="menu" class="peer-checked/menu:hidden z-50">
            <Icon icon=BiMenuRegular attr:class="text-white ml-auto h-[1.5em] w-[1.5em] m-[0.25rem] md:h-[3rem] md:w-[3rem] md:m-[0.75rem] item-center" />
        </label>
        <label for="menu" class="hidden peer-checked/menu:block z-50">
            <Icon icon=IoCloseSharp attr:class="ml-auto h-[1.5em] w-[1.5em] m-[0.25rem] md:h-[3rem] md:w-[3rem] md:m-[0.75rem] item-center" />
        </label>
        <div class="w-full h-full fixed overflow-hidden max-w-0  \
                    peer-checked/menu:max-w-full z-40 box-shadow-lg shadow-color-white flex flex-row-reverse">
            <ul class="h-[100vh] w-[60vw] pt-[72px] dark:bg-black bg-white">
                <li><LightDarkSwitch /></li>
                <li>menu2</li>
                <li>menu3</li>
                <li>menu4</li>
            </ul>
        </div>
    }
}