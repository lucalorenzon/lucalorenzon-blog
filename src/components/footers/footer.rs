use leptos::prelude::*;

#[component]
pub fn BlogFooter() -> impl IntoView {
    view! {
        <footer class="bottom-0 z-50 sticky">
            <hr class="h-px"/>
            <span>@ Luca Lorenzon production</span>
        </footer>
    }
}
