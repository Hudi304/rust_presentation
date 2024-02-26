use leptos::{ leptos_dom::logging::console_log, * };
use leptos_router::*;

use crate::pages::{
    bilbiography::Bibliography,
    intro_01::Intro_01,
    intro_02::Intro_02,
    intro_04::TheBorrowChecker,
    intro_05::TheBorrowChecker2,
    memory_safety_03::MemorySafety_03,
};

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

            <Route path="/intro_02" view=|| view! { <Intro_02/> }/>
            <Route path="/intro_03" view=|| view! { <MemorySafety_03/> }/>
            <Route path="/intro_04" view=|| view! { <TheBorrowChecker/> }/>
            <Route path="/intro_05" view=|| view! { <TheBorrowChecker2/> }/>

            <Route path="/bibliography" view=|| view! { <Bibliography/> }/>



            <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>

        </Routes>
    }
}
