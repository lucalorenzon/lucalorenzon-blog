use leptos::prelude::*;

use crate::pages::listing_page::ListingPage;

/// HOME-PAGE: the chronological special case of LISTING-PAGE (AC-3) — same
/// content, different route (`/`). A thin wrapper, not a duplicate: today
/// there is no pagination/filtering to differentiate them.
#[component]
pub fn HomePage() -> impl IntoView {
    view! { <ListingPage/> }
}
