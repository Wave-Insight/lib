#![deny(
    // The following are allowed by default lints according to
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    anonymous_parameters,
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths, // allow anonymous lifetime
    missing_copy_implementations, // Copy may cause unnecessary memory copy
    missing_debug_implementations,
    missing_docs, // TODO: add documents
    single_use_lifetimes, // TODO: fix lifetime names only used once
    trivial_casts, // TODO: remove trivial casts in code
    trivial_numeric_casts,
    unreachable_pub, // allow clippy::redundant_pub_crate lint instead
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results, // TODO: fix unused results
    variant_size_differences,

    warnings, // treat all wanings as errors

    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(
    // Some explicitly allowed Clippy lints, must have clear reason to allow
    clippy::blanket_clippy_restriction_lints, // allow clippy::restriction
    clippy::implicit_return, // actually omitting the return keyword is idiomatic Rust code
    clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    clippy::panic, // allow debug_assert, panic in production code
    // clippy::panic_in_result_fn,
    clippy::missing_errors_doc, // TODO: add error docs
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::missing_panics_doc, // TODO: add panic docs
    clippy::panic_in_result_fn,
    clippy::print_stdout,
    clippy::use_debug
)]
// #![deny(missing_docs)]

//! Hello world example for Rust.
// use wave_insight_lib::structure::{Structure, Constructor};
// use wave_insight_lib::shell;
// use wave_insight_lib::app::INFO;
use wave_insight_lib::app::App;
// use clap::{Arg, App};
// use clap::{Arg, Command};


fn main() {
    // let (mut app_shell, app_id) = shell::init();
    let mut app = App::init();
    app.greeting();
    // let matches = cmd.get_matches();
    // if let Some(file_path) = matches.get_one::<String>("input_file"){
    //     let _ = app.open_vcd(file_path);
    // }
    // if let Some(file_path) = matches.get_one::<String>("tcl_script"){
    //     // let _ = app::open_vcd(&mut app_shell, app_id, file_path);
    //     app.shell.script(&vec![file_path.to_owned()][0..]);
    // }
    // NEXT, evaluate the file, if any.

    // if args.len() > 1 {
    //     molt_shell::script(&mut app_shell, &args[1..]);
    // } else {
    // }
    app.matech();
    app.shell.repl();
}
