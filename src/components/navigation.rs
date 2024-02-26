use leptos::*;

use crate::components::{ nav_button::NavButton, page_layout::PageLayout };

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <div class="flex flex-col overflow-hidden bg-gray-700 rounded-none gap-3 p-3 w-[200px]">
            <NavButton>"Intro 1"</NavButton>
            <NavButton>"Intro 2"</NavButton>
            <NavButton>"Intro 3"</NavButton>
            <NavButton>"Intro 4"</NavButton>
        </div>
    }
}
