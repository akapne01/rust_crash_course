mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_references;
mod print;
mod slice_type;
mod string_slices;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;
mod rectangles_struct_example;
mod optional_enum;

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
    rectangles_struct_example::run();
    optional_enum::run();
}
