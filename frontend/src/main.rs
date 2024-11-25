mod pages;

use crate::pages::home::home::Home;
use leptos::*;
use leptos_router::{Route, Router, Routes};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                </Routes>
            </main>
        </Router>
    }
}
