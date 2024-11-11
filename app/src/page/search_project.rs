use leptos::IntoView;
use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn SearchProject() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <div class="m-2">
            <h1 class="text-2xl">EN</h1>
            <h1 class="text-2xl">Search</h1>
        </div>
    }
}