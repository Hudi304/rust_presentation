mod app;
mod pages;
mod components;

use leptos::{ mount_to_body, view };
use leptos_router::*;
use components::main_routes::PresentationRoutes;

use crate::components::{ custom_route::CustomRoute, navigation::Navigation };

use leptos_icons::*;
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

                        <div class="flex items-center justify-between p-4">

                            <Icon
                                class="text-gray-500 hover:text-gray-400 transition-all mr-3"
                                icon=icondata::BiMenuRegular
                                width="30px"
                                height="30px"
                            />

                            <div class="flex items-center justify-center text-3xl">
                                "Not a faster Horse, but a new way of programming."
                            </div>

                            <Icon
                                class="text-gray-500 hover:text-gray-400 transition-all mr-3"
                                icon=icondata::BiMenuRegular
                                width="30px"
                                height="30px"
                            />

                        </div>

                        <PresentationRoutes/>
                    </div>
                </Router>

            </div>
        }
    });
}
