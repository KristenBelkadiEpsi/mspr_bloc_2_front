use leptos::{mount::mount_to_body, *};
use mspr_bloc_2_front::App;


fn main() {
    mount_to_body(|| view! { <App /> })
}