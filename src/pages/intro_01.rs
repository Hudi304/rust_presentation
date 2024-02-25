use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn Intro_01(#[prop(into)] cb: Callback<i32, ()>) -> impl IntoView {
    cb.call(4);

    let count = use_context::<ReadSignal<i32>>().expect("there to be a `count` signal provided");

    view! {
        <PageLayout>
            <div>"Intro " {count}</div>
        </PageLayout>
    }
}
