use leptos::*;

use crate::components::page_layout::PageLayout;

#[component]
pub fn MemorySafety_03() -> impl IntoView {
    view! {
        <PageLayout>
            <div class="text-orange-500 text-7xl font-bold">"Memory safety"</div>

            <div class="grid  grid-cols-[100px_auto_auto] grid-rows-[30px_auto_auto] border">
                <span>""</span>
                <span class="border">"Memory Safe"</span>
                <span class="border">"NOT Memory Safe"</span>
                <span class="border">"Garbage Collected"</span>
                <span class="grid grid-cols-3 grid-rows-3 gap-3 p-3 border">
                    <img class="h-20 " src="public/language_logos/c_sharp2.svg"/>
                    <img class="h-20" src="public/language_logos/js.png"/>
                    <img class="h-20" src="public/language_logos/ts.png"/>
                    <img class="h-20" src="public/language_logos/swift.png"/>
                    <img class="h-20 bg-white rounded-md" src="public/language_logos/java.png"/>
                    <img class="h-20" src="public/language_logos/python.png"/>
                    <img class="h-20 col-span-2" src="public/language_logos/Go.png"/>
                </span>
                <span class="border flex items-center justify-center text-5xl text-red-500">
                    "X"
                </span>

                <span class="border">"NOT Garbage Collected"</span>

                <span class="flex gap-3 p-3 border">
                    <img class="h-20" src="public/language_logos/rust.png"/>

                    <span class="bg-white p-3 rounded-md">
                        <img class="h-14  " src="public/language_logos/zig.png"/>
                    </span>

                </span>

                <span class="flex gap-3 p-3 border">
                    <img class="w-20 h-20" src="public/language_logos/Cpp.png"/>
                    <img class="w-20 h-20" src="public/language_logos/C.png"/>
                    "add .asm"
                </span>

            </div>
        </PageLayout>
    }
}
