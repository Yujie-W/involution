// Function to display version number
pub fn show_version_info() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("\nWelcome to Involution v{}!", VERSION);
}
