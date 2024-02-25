use leptos::*;

#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <div class="bg-slate-500 p-3 rounded-md shadow-md">

            {children()}
        </div>
    }
}
