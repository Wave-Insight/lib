//! Sample Molt Extension
//! shell: Wave-Insight Shell 
use std::fs::OpenOptions;
use std::io::Write;
use std::process;
use colored::Colorize;
use molt::{molt_ok,molt_err};
use molt::Interp;
use molt::types::*;
use crate::app;
use crate::app::AppContext;

/// Hello world example for Rust.
#[allow(missing_debug_implementations)]
#[derive(Clone, Copy)]
pub struct Command{
    /// Hello world example for Rust.
    pub name: &'static str,
    /// Hello world example for Rust.
    pub opts: &'static str,
    /// Hello world example for Rust.
    pub info: &'static str,
    /// Hello world example for Rust.
    pub func: &'static CommandFunc,
}

/// All Commands Defination.
pub const COMMANDS: [Command;5] = [
    Command{
        name : "version",
        opts : "",
        info : "Outputs Wave-Insight version information",
        func : &CMD_VERSION,
    },
    Command{
        name : "help",
        opts : "",
        info : "Outputs this command line help message",
        func : &CMD_HELP,
    },
    Command{
        name : "prompt",
        opts : "",
        info : "Outpus prompt (internal function)",
        func : &CMD_PROMPT,
    },
    Command{
        name : "open_vcd",
        opts : "input_file",
        info : "Open a .vcd (Value Change Dump) file",
        func : &CMD_OPEN_VCD,
    },
    Command{
        name : "test",
        opts : "",
        info : "Testing Only",
        func : &CMD_TEST,
    },
];
/// Hello world example for Rust.
#[allow(missing_debug_implementations)]
pub struct Wish{
    /// Print greeting infomation
    pub shell: Interp,
    /// Print greeting infomation
    pub ctx_id: ContextID,
    /// Print greeting infomation
    pub have_greeting: bool,
}
impl Wish {
    /// Install the extension's commands into the Interp.
    pub fn init() -> Self{
        let mut shell = Interp::new();
        let ctx_id = shell.save_context(AppContext::new());
        for command in COMMANDS {
            shell.add_context_command(command.name, command.func.to_owned(),ctx_id);
        }
        if let Ok(_)=shell.set_var(&Value::from("tcl_prompt1"), Value::from("return \"[prompt]\"")){
        }else{
            eprintln!("{} Could not install.", "error:".red().bold());
            process::exit(1);
        }
        // app::greeting();
        // NEXT, load the extension's Tcl code
        if let Err(exception) = shell.eval(include_str!("shell.tcl")) {
            panic!("Error in benchmark Tcl library: {}", exception.value().as_str());
        }
        return Self { 
            shell,
            ctx_id,
            have_greeting:false,
        }
    }
    /// Install the extension's commands into the Interp.
    pub fn context(&mut self) -> &mut AppContext{
        self.shell.context::<AppContext>(self.ctx_id)
    }
    /// Install the extension's commands into the Interp.
    pub fn script(&mut self, args: &[String]){
        molt_shell::script(&mut self.shell, args);
    }
    /// Install the extension's commands into the Interp.
    pub fn repl(&mut self){
        molt_shell::repl(&mut self.shell);
    }
}

const CMD_HELP: CommandFunc = |_: &mut Interp, _: ContextID, _: &[Value]| -> MoltResult {
    app::App::help();
    molt_ok!()
};

const CMD_PROMPT: CommandFunc = |shell: &mut Interp, ctx_id: ContextID, _: &[Value]| -> MoltResult {
    let prompt = app::prompt(shell, ctx_id)?;
    molt_ok!(prompt)
};

const CMD_VERSION: CommandFunc = |_: &mut Interp, _: ContextID, _: &[Value]| -> MoltResult {
    molt_ok!(format!("WAVE-INSIGHT {} -- {}", app::INFO.version, app::INFO.time))
};

const CMD_OPEN_VCD: CommandFunc = |shell: &mut Interp, ctx_id: ContextID, argv: &[Value]| -> MoltResult {
    // Correct number of arguments?
    let _ = check_args(1, argv, 2, 2, "input_file")?;
    let file_path = argv[1].as_str();
    app::open_vcd(shell, ctx_id, remove_quotes(file_path))
};

const CMD_TEST: CommandFunc = |shell: &mut Interp, ctx_id: ContextID, _: &[Value]| -> MoltResult {
    let context = shell.context::<AppContext>(ctx_id);
    println!("{:?}",context);
    println!();
    if let Some(s) = &context.structure{
        // println!("{:?}",s.to_bincode());
        if let Ok(ref mut file) = OpenOptions::new()
            .write(true)
            .open("bincode1"){
                let e =file.write_all(&s.to_bincode());
                println!("{:?}",e)
            };
            
    }
    molt_ok!()
};

/// Remove Beginning and Ending Double/Single Quotes:
/// 
/// open_vcd "tests/input.vcd" <-> open_vcd 'tests/input.vcd' <-> open_vcd tests/input.vcd
#[inline]
fn remove_quotes(content:&str)-> &str{
    let mut content = content;
    if content.len() >= 2{
        let first = content.chars().nth(0);
        let last = content.chars().nth(content.len()-1);
        if first == last && (first == Some('\'') || first == Some('\"')){
            if let Some(content_) = content.get(1..content.len()-1){
                content = content_;
            }
        }
    }
    return content
}
/// Overwrite check_args
pub fn check_args(
    namec: usize,
    argv: &[Value],
    min: usize,
    max: usize,
    argsig: &str,
) -> MoltResult {
    assert!(namec >= 1);
    assert!(min >= 1);
    assert!(!argv.is_empty());

    if argv.len() < min || (max > 0 && argv.len() > max) {
        let cmd_tokens = Value::from(&argv[0..namec]);
        molt_err!(
            "{} Args should be \"{} {}\"",
            "wrong:".bold().red(),
            cmd_tokens.to_string(),
            argsig.bold().italic(),
        )
    } else {
        molt_ok!()
    }
}