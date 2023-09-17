fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    fsd_builder::run_app(); // Call the run_app function from your library crate
}
