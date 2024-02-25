use leptos::*;
use leptos_router::*;

#[component]
pub fn PresentationRoutes() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=|| view! { <div>"home"</div> }/>
            <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
    }
}
