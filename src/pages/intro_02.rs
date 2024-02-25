use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn Intro_02() -> impl IntoView {
    view! {
        <PageLayout>
            <div class="text-orange-500 text-7xl font-bold">"What is Rust"</div>
            <div class="">"General purpose Language = from embedded to high level applications"</div>
            <div class="">"Statically typed = the type of each and every variable must be known at compile time!"</div>
            <div class="">"Memory Safe = ALL references are valid* "</div>
            <div class="">"Multi Paradigm = "</div>
        </PageLayout>
    }
}
