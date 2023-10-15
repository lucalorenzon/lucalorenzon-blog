use leptos::*;

#[component]
pub fn BlogFooter() -> impl IntoView {
    view! {
        <hr class="h-px"/>
        <footer class="bottom-0">
            <span>@ Luca Lorenzon production</span>
        </footer>
    }
}