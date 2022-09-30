use std::{
    path::Path,
    process,
};
use clap::{Command, Arg, ArgMatches};
// use molt::{Interp, ContextID, Value};
// use molt::check_args;
use molt::{molt_ok,molt_err};
use molt::Interp;
use molt::types::*;

use colored::Colorize;
use crate::structure;
use crate::wish;
/// Print greeting infomation
#[allow(missing_debug_implementations)]
pub struct App{
    /// info: AppInfo,
    pub wish: wish::Wish,
    /// info: AppInfo,
    pub args: ArgMatches,
}

impl App {
    const DEFAULT_TCL_SCRIPT: &str = "wave-insight.tcl";
    /// Print greeting infomation
    pub fn init()->Self{
        let args = Command::new(INFO.name)
            .bin_name("wave-insight")
            .version(INFO.version)
            .about("A better open source wave viewer.")
            .subcommand_required(false)
            .arg(Arg::new("input_file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .value_parser(clap::value_parser!(String))
                .help("Provides an input wave file to the program")
            )
            .arg(Arg::new("tcl_script")
                .short('s')
                .long("script")
                .value_name("FILE")
                .default_value(Self::DEFAULT_TCL_SCRIPT)
                .value_parser(clap::value_parser!(String))
                .help("Provides a TCL script file to the program")
            )
            .arg(Arg::new("token")
                .short('t')
                .long("token")
                .value_name("STRING")
                // .default_value(Self::DEFAULT_TCL_SCRIPT)
                .value_parser(clap::value_parser!(String))
                .help("Provides an auth token")
            )
            .arg(Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .value_parser(clap::value_parser!(u16).range(3000..))
                // .action(ArgAction::Set)
                .help("Provides a port number for http")
            )
            .get_matches();
        Self{
            wish: wish::Wish::init(),
            args,
        }
    }
    /// # matech
    /// 
    /// Matech flags with input argu
    #[inline]
    pub fn matech_args(&mut self) {
        if let Some(file_path) = self.args.get_one::<String>("tcl_script"){
            if Path::new(file_path).exists(){
                self.greeting();
                self.wish.have_greeting = true;
                println!();
                if file_path == Self::DEFAULT_TCL_SCRIPT{
                    println!("{} | Executing {} TCL script: '{}'", INFO.name.bold(), "default".underline().bold(), file_path);
                } else {
                    println!("{} | Executing TCL script: '{}'", INFO.name.bold(), file_path);
                }
                self.wish.script(&vec![file_path.to_owned()][0..]);
            } else if file_path != Self::DEFAULT_TCL_SCRIPT {
                eprintln!("{} No such tcl script file: {}", "error:".red().bold(), file_path);
                process::exit(1);
            }
        }
        // self.args.
        if let Some(file_path) = self.args.get_one::<String>("input_file"){
            if Path::new(file_path).exists(){
                self.greeting();
                self.wish.have_greeting = true;
                println!();
                println!("{} | Open VCD file: '{}'", INFO.name.bold(), file_path);
                let _ = open_vcd(&mut self.wish.shell, self.wish.ctx_id, file_path);
            } else if file_path != Self::DEFAULT_TCL_SCRIPT {
                eprintln!("{} No such input file: {}", "error:".red().bold(), file_path);
                process::exit(1);
            }
        }
        println!()
    }
    /// # greeting
    /// 
    /// Print greeting infomation
    #[inline]
    pub fn greeting(&self) {
        if !self.wish.have_greeting {
            println!("{}",format!("*** {} v{} -- {} ***", INFO.name, INFO.version, INFO.time).bold());
            println!("  * Wave Insight Shell (WISH): Tcl Interpreter");
            println!("  * {}", INFO.url);
        }
    }

    /// # help
    /// 
    /// Print help infomation
    #[inline]
    pub fn help() {
        println!("{}",format!("*** {} {} -- {} ***", INFO.name, INFO.version, INFO.time).bold());
    }
}

/// Hello world example for Rust.
pub const INFO: AppInfo = AppInfo{
    name: "Wave-Insight",
    version: env!("CARGO_PKG_VERSION"),
    url: env!("CARGO_PKG_HOMEPAGE"),
    // https://docs.rs/chrono/latest/chrono/format/strftime/
    time: build_time::build_time_local!("%b %d, %Y"),
};
/// Hello world example for Rust.
#[derive(Debug, Clone, Copy)]
pub struct AppInfo{
    /// Hello world example for Rust.
    pub name: &'static str,
    /// Hello world example for Rust.
    pub version: &'static str,
    /// Hello world example for Rust.
    pub url: &'static str,
    /// Hello world example for Rust.
    pub time: &'static str,
}
/// Hello world example for Rust.
#[derive(Debug)]
pub struct AppContext {
    /// Hello world example for Rust.
    pub structure: Option<structure::Structure>,
}
impl AppContext {
    /// New AppData
    pub fn new() -> Self{
        let structure = None;
        Self{
            structure,
        }
    }
}



/// # prompt
/// 
/// Change prompt
#[inline]
pub fn prompt(app_shell: &mut Interp, app_id: ContextID) -> Result<&str, Exception> {
    let _ = app_shell.context::<AppContext>(app_id);
    let prompt = "wish > ";
    Ok(prompt)
}

/// # open_vcd
/// 
/// Open a .vcd (Value Change Dump) file, see Chapter 18 of [IEEE Std 1364™-2005](https://www.eg.bucknell.edu/~csci320/2016-fall/wp-content/uploads/2015/08/verilog-std-1364-2005.pdf)
#[inline]
pub fn open_vcd(app_shell: &mut Interp, app_id: ContextID, file_path: &str) -> MoltResult {
    let context = app_shell.context::<AppContext>(app_id);
    match crate::parser::vcd(file_path) {
        Ok(c) => {
            context.structure = Some(c.structure);
            molt_ok!("ok")
        },
        Err(err) => molt_err!("{} {}: {}","error:".red().bold(),file_path, err),
    }
}