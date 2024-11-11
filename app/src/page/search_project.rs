use leptos::IntoView;
use leptos::prelude::*;
use crate::i18n::*;
use leptos_router::components::A;
use leptos_i18n::t;

/// Renders the home page of your application.
#[component]
pub fn SearchProject() -> impl IntoView {
    // Creates a reactive value to update the button
    let i18n = use_i18n();

    let l = move || i18n.get_locale().as_str();

    view! {
        <div class="m-2">
            <h1 class="text-2xl">{l()}</h1>
            <h1 class="text-2xl">{t!(i18n,search_project.search_project_title)}</h1>
            {[Locale::en,Locale::nl,Locale::fr].into_iter().map(|l| view!{
                                <A href=format!("/{}/{}",l.as_str(),td_string!(l,common.menu.search_project_path)) {..} class="-mx-3 block rounded-lg px-3 py-2.5 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">{l.as_str()}</A>
                            }).collect_view()}
        </div>
    }
}