// use leptos::IntoView;

#[derive(Clone)]
pub struct CustomRoute {
    pub path: &'static str,
    pub name: &'static str,
    // pub component: F,
}
impl CustomRoute {
    pub fn new(path: &'static str, name: &'static str) -> Self {
        CustomRoute { path, name }
    }
}
