use leptos::*;

use super::custom_route::CustomRoute;

#[component]
pub fn NavButton(children: Children, route: CustomRoute) -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    view! {
        <button
            class="text-gray-500 hover:text-blue-500 shadow-none font-bold flex px-1"
            on:click=move |_| navigate(route.path, Default::default())
        >
            {children()}
        </button>
    }
}
