mod math;
mod meta;

use meta::navigation::{choose_module, show_navigation};
use meta::version::show_version_info;

fn main() {
    // Display version number of the
    show_version_info();

    // Show the navigation in a while loop
    let mut breaking = false;
    while !breaking {
        show_navigation();
        breaking = choose_module();
    }
}
