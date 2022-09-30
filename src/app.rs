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
    pub shell: wish::Wish,
    /// info: AppInfo,
    pub args: ArgMatches,
}

impl App {
    /// Print greeting infomation
    pub fn init()->Self{
        let args = Command::new(INFO.name)
        .bin_name("wave-insight")
        .version(INFO.version)
        .about("package manager utility")
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
            .value_parser(clap::value_parser!(String))
            .help("Provides an tcl script file to the program")
        )
        .get_matches();
        Self{
            shell: wish::Wish::init(),
            args,
        }
    }
    /// # greeting
    /// 
    /// Print greeting infomation
    #[inline]
    pub fn matech(&mut self) {
        if let Some(file_path) = self.args.get_one::<String>("input_file"){
            let _ = open_vcd(&mut self.shell.shell, self.shell.ctx_id, file_path);
        }
        if let Some(file_path) = self.args.get_one::<String>("tcl_script"){
            // let _ = app::open_vcd(&mut app_shell, app_id, file_path);
            self.shell.script(&vec![file_path.to_owned()][0..]);
        }
    }
    /// # greeting
    /// 
    /// Print greeting infomation
    #[inline]
    pub fn greeting(&self) {
        println!("{}",format!("*** {} {} -- {} ***", INFO.name, INFO.version, INFO.time).bold());
        println!("  * Wave Insight Shell (WISH): Tcl Interpreter");
        println!("  * {}", INFO.url);
    }

    /// # open_vcd
    /// 
    /// Open a .vcd (Value Change Dump) file, see Chapter 18 of [IEEE Std 1364™-2005](https://www.eg.bucknell.edu/~csci320/2016-fall/wp-content/uploads/2015/08/verilog-std-1364-2005.pdf)
    // #[inline]
    // pub fn open_vcd(&mut self, file_path: &str) -> MoltResult {
    //     let mut context = self.shell.context();
    //     match crate::parser::vcd(file_path) {
    //         Ok(c) => {
    //             context.structure = Some(c.structure);
    //             molt_ok!("ok")
    //         },
    //         Err(e) => molt_err!(
    //             "error * {}: {}",file_path,e
    //         ),
    //     }
    // }

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
    time: build_time::build_time_local!("%b %m, %Y"),
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
        Err(e) => molt_err!(
            "error * {}: {}",file_path,e
        ),
    }
}