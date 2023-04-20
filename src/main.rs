mod math;
mod meta;

fn main() {
    // display version number of the
    meta::version::show_version_info();

    // run the math quiz
    math::plus::quiz(50);
}
