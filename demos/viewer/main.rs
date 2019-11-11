mod app;
mod samples;

use app::*;

#[macro_use]
extern crate lazy_static;

use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    let screen = Vector::new(800, 600);
    run_with("Teapot demo", screen, Settings::default(), || Application::new(screen));
}
