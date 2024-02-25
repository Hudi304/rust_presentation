mod app;
mod components;

use app::*;
use leptos::*;
use components::main_routes::PresentationRoutes;

fn main()
{
    mount_to_body(|| {
        view! {
            <Router>
               <PresentationRoutes
            </Router>
        }
    });
}
