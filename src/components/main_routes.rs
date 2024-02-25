use leptos::{ leptos_dom::logging::console_log, * };
use leptos_router::*;

use crate::pages::intro_01::Intro_01;

#[component]
pub fn PresentationRoutes() -> impl IntoView {
    let intro_cb = move |x: i32| console_log(&format!("Intro_01 cb: {}", x));

    let (count, set_count) = create_signal(0);

    provide_context(count);

    view! {
        <Routes>
            <Route path="/" view=|| view! { <div>"home"</div> }/>

            <Route
                path="/intro_01"
                view=|| {
                    view! { <Intro_01 cb=|x: i32| console_log(&format!("Intro_01 cb: {}", x))/> }
                }
            />

            <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>

        </Routes>
    }
}
