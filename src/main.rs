mod app;
mod pages;
mod components;

use leptos::{mount_to_body, view};
use leptos_router::*;
use components::main_routes::PresentationRoutes;

fn main() {
    mount_to_body(|| {
        view! {
            <div class="w-screen h-screen bg-blue-950 flex items-center justify-center">
                <Router>
                    <PresentationRoutes   />
                </Router>
            </div>
        }
    });
}
