use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn TheBorrowChecker2() -> impl IntoView {
    view! {
        // <PageLayout>
            <div class="text-orange-500 text-7xl font-bold">"The Borrow Checker"</div>
            <div>"Two rules"</div>
            <div>"1) Data has one owner"</div>
            <div>"2) Data can have multiple readers and one writer."</div>
        // </PageLayout>
    }
}
 