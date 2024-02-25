use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn Intro_01(#[prop(into)] cb: Callback<i32, ()>) -> impl IntoView {
    cb.call(4);

    let count = use_context::<ReadSignal<i32>>().expect("there to be a `count` signal provided");

    view! {
        <PageLayout>
            // <img src="public/rust.png" class="w-1/2" alt="Leptos logo"/>
            <div class="text-orange-500 text-7xl font-bold">"Rust"</div>
            <div class="">"A completely new way of programming!"</div>
        </PageLayout>
    }
}
