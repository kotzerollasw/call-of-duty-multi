mod features;
mod logger;

pub use features::Features;
pub use logger::init;

pub fn start_application() {
    init();
    let app = crate::main();
}