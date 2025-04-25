use leptos::html::*;
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        logging::log!("Tentative de connexion avec email: {}", email.get());
    };

    view! {
        <div class="login-container">
            <h2>"Connexion"</h2>
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        required
                    />
                </div>
                <div class="form-group">
                    <label for="password">"Mot de passe"</label>
                    <input
                        type="password"
                        id="password"
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        required
                    />
                </div>
                <button type="submit">"Se connecter"</button>
            </form>
        </div>
    }
} 