// use molt::{Interp, ContextID, Value};
// use molt::check_args;
// use molt::molt_ok;
use molt::Interp;
use molt::types::*;

use colored::Colorize;


use crate::structure;

/// Hello world example for Rust.
#[derive(Debug)]
pub struct AppContext { 
    /// Hello world example for Rust.
    pub info_name: String,
    /// Hello world example for Rust.
    pub info_version: String,
    /// Hello world example for Rust.
    pub info_url: String,
    /// Hello world example for Rust.
    pub info_time: String,
    /// Hello world example for Rust.
    pub info_command: Vec<[String;2]>,
    /// structure
    pub data_structure: Option<structure::Structure>,
}

impl AppContext {
    /// Hello world example for Rust.
    pub fn new() -> Self{
        let info_name = "Wave-Insight".to_string();
        let info_version = env!("CARGO_PKG_VERSION").to_string();
        let info_url = env!("CARGO_PKG_HOMEPAGE").to_string();
        // https://docs.rs/chrono/latest/chrono/format/strftime/
        let info_time = build_time::build_time_local!("%b %m, %Y").to_string();
        Self{
            info_name,
            info_version,
            info_url,
            info_time,
            info_command:vec![],
            data_structure: None
        }
    }
    /// Hello world example for Rust.
    pub fn add_command_info(&mut self, func_name: &str, info: &str){
        self.info_command.push([func_name.to_string(),info.to_string()]);
    }
}

/// # greeting
/// 
/// Print greeting infomation
#[inline]
pub fn greeting(app_shell: &mut Interp, app_id: ContextID) {
    let app = app_shell.context::<AppContext>(app_id);
    // let x =format!("*** {} {} -- {} ***", app.info_name, app.info_version, app.info_time).bold();
    println!("{}",format!("*** {} {} -- {} ***", app.info_name, app.info_version, app.info_time).bold());
    // println!("*** {} {} -- {} ***", app.info_name, app.info_version, app.info_time);
    println!("    {}", app.info_url);
}

/// # help
/// 
/// Print help infomation
#[inline]
pub fn help(app_shell: &mut Interp, app_id: ContextID) {
    let app = app_shell.context::<AppContext>(app_id);
    // let x =format!("*** {} {} -- {} ***", app.info_name, app.info_version, app.info_time).bold();
    println!("{}",format!("*** {} {} -- {} ***", app.info_name, app.info_version, app.info_time).bold());
    // println!("*** {} {} -- {} ***", app.info_name, app.info_version, app.info_time);
    println!("    {}", app.info_url);
}

/// # prompt
/// 
/// Change prompt
#[inline]
pub fn prompt(app_shell: &mut Interp, app_id: ContextID) -> Result<&str, Exception> {
    let _ = app_shell.context::<AppContext>(app_id);
    let prompt = "tcl1> ";

    // app_shell.set_var(&Value::from("tcl_prompt1"), Value::from(format!("return \"{}\"",prompt)))?;
    Ok(prompt)
}

/// # open_vcd
/// 
/// Open a .vcd (Value Change Dump) file, see Chapter 18 of [IEEE Std 1364™-2005](https://www.eg.bucknell.edu/~csci320/2016-fall/wp-content/uploads/2015/08/verilog-std-1364-2005.pdf)
#[inline]
pub fn open_vcd(app_shell: &mut Interp, app_id: ContextID, file_path: &str) {
    let app = app_shell.context::<AppContext>(app_id);
    match crate::parser::vcd(file_path) {
        Ok(c) => {
            app.data_structure = Some(c.structure);
            println!("ok")
        },
        Err(e) => eprintln!(" **error** {}: {}",e, file_path),
    }
}