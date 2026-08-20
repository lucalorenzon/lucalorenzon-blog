use leptos::prelude::*;
use leptos_use::breakpoints_tailwind;
use leptos_use::BreakpointsTailwind::{Md};

#[component]
pub fn AsciiArtLogo_(media_query: &'static str) -> impl IntoView {
    view! {
<pre class=move || format!("text-7px font-mono font-thin leading-none tracking-tighter p-5 {}", media_query) >
"`7MMF'           `7MM          ,.-==-.
  MM               MM       ,pd'      `g.
  MM    `7M'   `MF'MM  ,MP',P   ,dMb.A  Y. M******A'pd\"\"b.  `7Mb,od8
  MM      VA   ,V  MM ;Y  ,P   dP  ,MP  j8 Y     A'(O)  `8b   MM' \"'
  MM      ,VA ,V   MM;Mm  8:  dM'  dM   d'      A'      ,89   MM
  MM     ,M VVV    MM `Mb.Wb  YML.dML..d'      A'     \"\"Yb.   MM
.JMMmmmmMMM  W   .JMML. YA.Wb  ``\"\"^`\"'       A'         88 .JMML.
                            `M..     .,!     A'    (O)  .M'
                              `Ybmmd'       A'      bmmmd'"
</pre>
    }
}

#[component]
pub fn AsciiArtLogo(media_query: &'static str) -> impl IntoView {
    view! {
<pre class=move || format!("text-white text-[7pt] font-mono font-thin leading-none tracking-tighter p-2 {}", media_query) >
"    __        __   ______  __________
   / / _   __/ /__/ ____ \\/__  /__  /_____
  / / | | / / //_/ / __ `/  / / /_ </ ___/
 / /__| |/ / ,< / / /_/ /  / /___/ / /
/_____/___/_/\\_|\\ \\__,_/  /_//____/_/
                 \\____/                   "
</pre>
    }
}

#[component]
pub fn TextLogo(media_query: &'static str) -> impl IntoView {
    view! {
<p class=move || format!("text-white text-xl font-sans italic font-thin leading-[2.0rem] {}", media_query)>Lvk@73r</p>
    }
}

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <TextLogo media_query="block md:hidden" />
        <AsciiArtLogo media_query="hidden md:block" />
    }
}

#[component]
pub fn SmallImageIcon() -> impl IntoView {
    let breakpoints = breakpoints_tailwind();
    let md_width = *breakpoints.get(&Md).expect("It's there!");
    view! {
        <picture class="p-2">
            <source media=move || format!("(min-width: {}px)", md_width) srcset="/assets/logoIcon.png" />
            <img src="/assets/logoIconSmall.png" />
        </picture>
    }
}