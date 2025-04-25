use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use leptos::html::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::*;

#[component]
pub fn Register() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (qr_code, set_qr_code) = signal(None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if password.get() != confirm_password.get() {
            logging::log!("Les mots de passe ne correspondent pas");
            return;
        }
        logging::log!("Tentative d'inscription avec email: {}", email.get());
        let email = email.get();
        spawn_local(async move {
            
            let client = reqwest::Client::new();
            let qr = client
                .post("http://172.16.150.30:8080/function/qrcode-go")
                .header("content-type", "text/plain")
                .header("Access-Control-Allow-Origin", "null")
                .body(email)
                .send()
                .await
                .expect("impossible de recuperer le qrcode");
            logging::log!("la requête à été faite !");
            let bytes = qr.bytes().await.unwrap();
            logging::log!("{:?}", bytes);
            set_qr_code.set(Some(bytes));
        });
    };

    view! {
        <div class="register-container">
            <image src=move || {
                let bytes = qr_code.get().map(|b| b.into()).unwrap_or(Vec::new());
                BASE64_STANDARD.encode(bytes)
            } />

            <h2>"Inscription"</h2>
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
                <div class="form-group">
                    <label for="confirm_password">"Confirmer le mot de passe"</label>
                    <input
                        type="password"
                        id="confirm_password"
                        prop:value=confirm_password
                        on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                        required
                    />
                </div>
                <button type="submit">"S'inscrire"</button>
            </form>
        </div>
    }
}
