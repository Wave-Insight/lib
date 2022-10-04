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
    unused_crate_dependencies,
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
    // clippy::blanket_clippy_restriction_lints, // allow clippy::restriction
    // clippy::implicit_return, // actually omitting the return keyword is idiomatic Rust code
    // clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    // clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    // clippy::panic, // allow debug_assert, panic in production code
    // clippy::panic_in_result_fn,
    // clippy::missing_errors_doc, // TODO: add error docs
    // clippy::exhaustive_structs,
    // clippy::exhaustive_enums,
    // clippy::missing_panics_doc, // TODO: add panic docs
    // clippy::panic_in_result_fn,
    // clippy::print_stdout,
    clippy::use_debug
)]

/*!
# Hello world example for Rust.
*/

use std::process;

use actix_web::{HttpServer, App};
use actix_web_static_files::ResourceFiles;
use colored::Colorize;

const INFO_NAME: &str = env!("CARGO_PKG_NAME");
const INFO_URL: &str = env!("CARGO_PKG_HOMEPAGE");
const INFO_VERSION: &str = env!("CARGO_PKG_VERSION");
// https://docs.rs/chrono/latest/chrono/format/strftime/
const INFO_DATE: &str = build_time::build_time_local!("%b %d, %Y");

fn greeting(){
    println!("{}",format!("*** {} v{} -- {} ***", INFO_NAME, INFO_VERSION, INFO_DATE).bold());
    println!("  * {}", INFO_URL);
}

// mod server;
#[allow(unused_imports, unreachable_pub, unused_results)]
mod generate{
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}


/// 启动服务器
async fn create_app() {
    let (addr, port) = ("0.0.0.0", 8080);
    match HttpServer::new(move || {
        App::new()
        .service(server::index)
        .service(ResourceFiles::new("/", generate::generate()))
    }).bind(format!("{}:{}", addr, port))
    {
        Ok(server)=>{
            if port==80{
                println!("{} Listen at http://{}", "Running:".green().bold(), addr);
            }else{
                println!("{} Listen at http://{}:{}", "Running:".green().bold(), addr, port);
            }
            let _ = server.run().await;
        },
        Err(e) =>{
            eprintln!("{} {}", "error:".red().bold(), e);
            process::exit(1);
        }
    };
}

#[actix_web::main]
async fn main() {
    greeting();
    // println!(":?",generate::generate());
    create_app().await;
    // app::create_app().await;
}