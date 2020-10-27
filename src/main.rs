#![allow(unused_assignments, unused_variables, unused_imports, dead_code)]

mod slides;
pub use slides::*;

fn main() {
    println!("Hello, audience!");

    titlepage();

    syntax();
    control_flow();

    structs();
    enums();
    result_and_option();

    traits();

    closures();

    unsafe {
        tooling();
        borrow_checker();
    }
}

extern "C" {
    fn tooling();
    fn borrow_checker();
}
