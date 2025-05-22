mod utils; // declares submodule utils.rs
mod math;
fn main() {
    utils::greet("Romestian");

    let a = math::add::sum(23,54);
    let b = math::mul::mul(23, 3);
    let c = math::add::sum(a, b);
    println!("{c}");
}
