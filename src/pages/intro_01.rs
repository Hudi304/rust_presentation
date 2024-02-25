use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn Intro_01(#[prop(into)] cb: Callback<i32, ()>) -> impl IntoView {
    cb.call(4);

    view! {
        <PageLayout>
            <div>"Intro"</div>
        </PageLayout>
    }
}
