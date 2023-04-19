pub fn show_version_info() {
    // display version number of emerald
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("\nWelcome to Involution v{}!\n", VERSION);
}
