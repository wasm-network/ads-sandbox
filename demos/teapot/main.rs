use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    let screen = Vector::new(1000, 600);
    println!("Teapot demo");
    // run_with("Theme Edit", screen, Settings::default(), || Application::new(screen));
}
