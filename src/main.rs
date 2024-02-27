mod app;
mod pages;
mod components;

use leptos::{ mount_to_body, view };
use leptos_router::*;
use components::main_routes::PresentationRoutes;

use crate::components::{ app_bar::AppBar, custom_route::CustomRoute, navigation::Navigation };

fn main() {
    let routes: Vec<CustomRoute> = vec![
        CustomRoute::new("/", "/"),
        CustomRoute::new("/intro", "Introduction"),
        CustomRoute::new("/intro_01", "Introduction 01"),
        CustomRoute::new("/intro_02", "Introduction 02"),
        CustomRoute::new("/intro_03", "Introduction 03"),
        CustomRoute::new("/chapter_01", "Chapter 01"),
        CustomRoute::new("/chapter_02", "Chapter 02"),
        CustomRoute::new("/chapter_03", "Chapter 03"),
        CustomRoute::new("/chapter_04", "Chapter 04"),
        CustomRoute::new("/chapter_05", "Chapter 05"),
        CustomRoute::new("/chapter_06", "Chapter 06"),
        CustomRoute::new("/chapter_07", "Chapter 07"),
        CustomRoute::new("/chapter_08", "Chapter 08"),
        CustomRoute::new("/chapter_09", "Chapter 09"),
        CustomRoute::new("/chapter_10", "Chapter 10"),
        CustomRoute::new("/conclusion", "Conclusion")
    ];

    mount_to_body(|| {
        view! {
            <div class="flex">
                <Router>

                    <Navigation routes=routes/>

                    <div class="w-screen h-screen bg-gray-900 grid grid-rows-[60px_auto]">
                        <AppBar/>

                        <PresentationRoutes/>
                    </div>
                </Router>

            </div>
        }
    });
}
