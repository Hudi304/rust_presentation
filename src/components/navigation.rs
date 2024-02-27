use leptos::*;

use crate::components::nav_button::NavButton;

use super::custom_route::CustomRoute;

#[component]
pub fn Navigation(routes: Vec<CustomRoute>) -> impl IntoView {
    let routes: Vec<_> = routes
        .into_iter()
        .map(|route| {
            view! { <NavButton route=route.clone()>{route.name}</NavButton> }
            // view! { <div >{route.name}</div> }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="flex flex-col overflow-hidden bg-gray-700 rounded-none gap-3 p-3 w-[200px]">
            {routes.collect_view()}
        </div>
    }
}
