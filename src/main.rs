mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_references;
mod structs;
mod enums;
mod cli;
mod slice_type;
mod string_slices;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_references::run();
    structs::run();
    enums::run();
    cli::run();
    slice_type::run();
    string_slices::run();
}
