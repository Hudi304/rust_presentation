use leptos::*;

pub enum PresentationRoutes {
    intro_01,
    intro_02,
    intro_03,
}
#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    fn navigate_forward(current: PresentationRoutes) -> PresentationRoutes {
        match current {
            PresentationRoutes::intro_01 => PresentationRoutes::intro_02,
            PresentationRoutes::intro_02 => PresentationRoutes::intro_03,
            PresentationRoutes::intro_03 => PresentationRoutes::intro_01,
        }
    }

    view! {
        <div class="flex justify-between h-full w-full gap-3">

            <button
                class="h-full w-20 bg-opacity-20 hover:bg-gray-600 
                transition-all font-extrabold"
                on:click=move |_| navigate("/", Default::default())
            >
                "<"
            </button>

            <div class="flex h-full items-center justify-center">
                <div class="bg-slate-500 p-3 rounded-md shadow-md">{children()}</div>
            </div>

            <button class="h-full w-20
            hover:bg-gray-600 
            border-none
            outline-none
            ring-0
            hover:bg-opacity-20 
            transition-all 
            font-extrabold">">"</button>

        </div>
    }
}
