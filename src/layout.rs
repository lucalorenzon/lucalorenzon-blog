use leptos::*;


use crate::components::headers::headers::BlogHeader;
use crate::components::footers::footer::BlogFooter;

#[component]
pub fn Layout( children: Children) -> impl IntoView {
    view! {
        <BlogHeader />
        <main class="flex-[1_0_auto] relative z-0 -mt-[80px]" >
            {children()}
        </main>
        <BlogFooter />
    }
}