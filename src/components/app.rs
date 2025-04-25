use leptos::html::*;
use leptos::prelude::*;

use crate::Auth;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Cofrap"</h1>
            <Auth />
        </div>
    }
}