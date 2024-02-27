use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn Intro_01() -> impl IntoView {
    view! {
        <PageLayout>
            <div class="text-orange-500 text-7xl font-bold">"Rust"</div>
            <div class="">"A completely new way of programming!"</div>
        </PageLayout>
    }
}
