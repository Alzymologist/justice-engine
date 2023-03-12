// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
//
mod web;
mod yaml;

fn main() {
    yew::Renderer::<web::App>::new().render();
}
