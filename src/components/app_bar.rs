use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn AppBar() -> impl IntoView {
    view! {
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
    }
}
