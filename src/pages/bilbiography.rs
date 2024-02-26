use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn Bibliography() -> impl IntoView {
    view! {
        <PageLayout>
            <div>"Bibliography"</div>

            <div class="flex flex-col">
                <span>"No Boilerplate"</span>

                <a href="https://www.youtube.com/watch?v=br3GIIQeefY">"Rust for the impatient"</a>

                <a href="https://www.youtube.com/watch?v=0rJ94rbdteE">
                    "Rust makes you feel like a GENIUS"
                </a>

                <span>"faster-than-lime"</span>
                <a href="https://www.youtube.com/watch?v=0rJ94rbdteE">
                    "A half-hour to learn Rust "
                </a>

            </div>
        </PageLayout>
    }
}
