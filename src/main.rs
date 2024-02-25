mod app;

use app::*;
use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <Routes>
                    <Route path="/" view=|| view! {<div>"home"</div>}/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </Router>
        }
    })
}
