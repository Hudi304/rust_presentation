use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            {children()}
        </button>
    }
}
