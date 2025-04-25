use leptos::html::*;
use leptos::prelude::*;

use crate::Login;
use crate::Register;

#[component]
pub fn Auth() -> impl IntoView {
    let (is_login, set_is_login) = signal(true);

    view! {
        <div class="auth-switch-container">
            <div class="auth-switch-buttons">
                <button
                    class=move || if is_login.get() { "active" } else { "" }
                    on:click=move |_| set_is_login.set(true)
                >
                    "Connexion"
                </button>
                <button
                    class=move || if !is_login.get() { "active" } else { "" }
                    on:click=move |_| set_is_login.set(false)
                >
                    "Inscription"
                </button>
            </div>
            <div class="auth-form-container">
                {move || {
                    if is_login.get() {
                        view! { <Login /> }.into_any()
                    } else {
                        view! { <Register /> }.into_any()
                    }
                }}
            </div>
        </div>
    }
} 