use iced::{
    Element,
    widget::{
        Container, Image, button, center, column, horizontal_space, image::Handle, row, text_input,
    },
};
use mspr_bloc2_front::service::get_qrcode;
use reqwest::StatusCode;
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    Login,
    Register,
    EmailChanged(String),
    PasswordChanged(String),
    ConfirmationPasswordChanged(String),
    ApiCallLogin,
    ApiCallRegister,
}

struct AuthApp {
    is_logging: bool,
    email: String,
    password: String,
    confirmation_password: String,
    qrcode: Option<Vec<u8>>,
}
impl Default for AuthApp {
    fn default() -> Self {
        Self {
            is_logging: false,
            email: String::new(),
            password: String::new(),
            confirmation_password: String::new(),
            qrcode: None,
        }
    }
}
impl AuthApp {
    #[tokio::main]
    async fn update(&mut self, message: Message) {
        match message {
            Message::Login => {
                self.is_logging = true;
            }
            Message::Register => {
                self.is_logging = false;
            }
            Message::EmailChanged(new_value) => {
                self.email = new_value;
            }
            Message::PasswordChanged(new_value) => {
                self.password = new_value;
            }
            Message::ConfirmationPasswordChanged(new_value) => {
                self.confirmation_password = new_value;
            }
            Message::ApiCallLogin => {
                // let qrcode_bytes = get_qrcode(self.email.clone()).await;
                // self.qrcode = Some(qrcode_bytes);
            }
            Message::ApiCallRegister => {
                let response = get_qrcode(self.email.clone()).await;
                match response.status() {
                    StatusCode::OK => {
                        let bytes: Vec<u8> = response.bytes().await.unwrap().into();
                        self.qrcode = Some(bytes);
                    }
                    _ => eprintln!("erreur lors de l'inscription !"),
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let mut column = column![];

        if self.is_logging {
            column = column
                .push(
                    text_input("addresse email", &self.email)
                        .on_input(|new_value| Message::EmailChanged(new_value)),
                )
                .push(
                    text_input("mot de passe", &self.password)
                        .secure(true)
                        .on_input(|new_value| Message::PasswordChanged(new_value)),
                );
        } else {
            column = column
                .push(
                    text_input("addresse email", &self.email)
                        .on_input(|new_value| Message::EmailChanged(new_value)),
                )
                .push(
                    text_input("mot de passe", &self.password)
                        .secure(true)
                        .on_input(|new_value| Message::PasswordChanged(new_value)),
                )
                .push(
                    text_input("répeter votre mot de passe", &self.confirmation_password)
                        .secure(true)
                        .on_input(|new_value| Message::ConfirmationPasswordChanged(new_value)),
                )
        }

        let row = row![
            button("inscription/connexion").on_press(if self.is_logging {
                Message::Register
            } else {
                Message::Login
            }),
            horizontal_space(),
            button(if self.is_logging {
                "se connecter"
            } else {
                "s'inscrire"
            })
            .on_press(if self.is_logging {
                Message::ApiCallLogin
            } else {
                Message::ApiCallRegister
            })
        ];
        if self.qrcode.is_some() {}
        column = column.push_maybe(
            self.qrcode
                .clone()
                .map(|v| Image::new(Handle::from_bytes(v))),
        );
        column = column.push(row);
        Container::new(center(column.padding(32).spacing(32))).into()
    }
}

fn main() -> iced::Result {
    iced::run("Auth app", AuthApp::update, AuthApp::view)
}
