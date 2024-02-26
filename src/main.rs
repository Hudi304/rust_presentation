mod app;
mod pages;
mod components;

use leptos::{ create_signal, mount_to_body, view };
use leptos_router::*;
use components::main_routes::PresentationRoutes;

use crate::components::navigation::Navigation;

use leptos_icons::*;

fn main() {
    let (is_menu_visible, set_is_menu_visible) = create_signal(true);

    mount_to_body(|| {
        view! {
            <div class="flex">

                <Navigation/>

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

                    <Router>
                        <PresentationRoutes/>
                    </Router>
                </div>
            </div>
        }
    });
}
