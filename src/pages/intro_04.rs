use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn TheBorrowChecker() -> impl IntoView {
    view! {
        // <PageLayout>
            <div class="text-orange-500 text-7xl font-bold">"The Borrow Checker"</div>

            <div>"RAII (Resource Acquisition Is Initialization) "</div>

            <div>
                "C++ programming technique which binds the life cycle of a resource
                that must be acquired before use  to the lifetime of an object."
            </div>

            <div class="flex gap-2">
                <span class="whitespace-nowrap">"resource = "</span>

                <div class="flex flex-col">
                    <span>"allocated heap memory"</span>
                    <span>"thread of execution"</span>

                    <span>"open socket"</span>
                    <span>"open file"</span>

                    <span>"locked mutex"</span>
                    <span>"disk space"</span>
                    <span>"database connection"</span>

                    "—anything that exists in limited supply"
                </div>
            </div>

        // </PageLayout>
    }
}
