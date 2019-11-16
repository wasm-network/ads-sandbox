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
    std::env::set_var("RUST_LOG", "trace,quicksilver=info");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(true).init();
    #[cfg(not(target_arch = "wasm32"))]
    color_backtrace::install();

    let screen = Vector::new(800, 600);
    run_with("Ad Viewer", screen, Settings::default(), || Application::new(screen));
}
