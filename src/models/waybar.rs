use serde::Serialize;

#[derive(Serialize)]
pub struct WaybarOutput {
    pub text: String,
    pub alt: String,
    pub tooltip: String,
    pub class: String,
}