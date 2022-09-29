//! Sample Molt Extension
//! WISH: Wave-Insight Shell 
use molt::check_args;
use molt::molt_ok;
use molt::Interp;
use molt::types::*;

use crate::app;
use crate::app::AppContext;
use crate::app::greeting;

// The AppContext

/// Install the extension's commands into the Interp.
pub fn new() ->Interp{
    Interp::new()
}

/// Install the extension's commands into the Interp.
pub fn install(app_shell: &mut Interp, app_id: ContextID) -> MoltResult {
    // Install a command implemented in Rust.
    let mut func_name: &str;
    let mut info: &str;
    let mut func: CommandFunc;

    func_name = "version";
    func = cmd_version;
    info = "Outputs Wave-Insight version information";
    app_shell.add_context_command(func_name, func,app_id);
    app_shell.context::<AppContext>(app_id).add_command_info(func_name, info);

    func_name = "help";
    func = cmd_help;
    info = "Outputs this command line help message";
    app_shell.add_context_command(func_name, func,app_id);
    app_shell.context::<AppContext>(app_id).add_command_info(func_name, info);

    func_name = "prompt";
    func = cmd_prompt;
    info = "Outpus prompt (internal function)";
    app_shell.add_context_command(func_name, func,app_id);
    app_shell.context::<AppContext>(app_id).add_command_info(func_name, info);
    app_shell.set_var(&Value::from("tcl_prompt1"), Value::from("return \"[prompt]\""))?;

    func_name = "open_vcd";
    func = cmd_open_vcd;
    info = "Open a .vcd (Value Change Dump) file";
    app_shell.add_context_command(func_name, func,app_id);
    app_shell.context::<AppContext>(app_id).add_command_info(func_name, info);

    app_shell.add_context_command("test", cmd_test, app_id);
    
    greeting(app_shell, app_id);
    // NEXT, load the extension's Tcl code
    if let Err(exception) = app_shell.eval(include_str!("shell.tcl")) {
        panic!("Error in benchmark Tcl library: {}", exception.value().as_str());
    }
    molt_ok!()
}
#[inline]
fn cmd_help(app_shell: &mut Interp, app_id: ContextID, _: &[Value]) -> MoltResult {
    app::help(app_shell, app_id);
    molt_ok!()
}
#[inline]
fn cmd_prompt(app_shell: &mut Interp, app_id: ContextID, _: &[Value]) -> MoltResult {
    let prompt = app::prompt(app_shell, app_id)?;
    molt_ok!(prompt)
}
#[inline]
fn cmd_version(app_shell: &mut Interp, app_id: ContextID, _: &[Value]) -> MoltResult {
    // Correct number of arguments?
    // let _ = check_args(1, argv, 1, 1, "")?;
    let app = app_shell.context::<AppContext>(app_id);
    // Return the result.
    molt_ok!(format!("WAVE-INSIGHT {} -- {}", app.info_version, app.info_time))
}
#[inline]
fn cmd_open_vcd(app_shell: &mut Interp, app_id: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    let _ = check_args(1, argv, 2, 2, "value")?;
    let file_path = argv[1].as_str();
    app::open_vcd(app_shell, app_id, file_path);
    molt_ok!()
}
#[inline]
fn cmd_test(app_shell: &mut Interp, app_id: ContextID, _: &[Value]) -> MoltResult {
    let app = app_shell.context::<AppContext>(app_id);
    println!("{:?}",app);
    molt_ok!()
}

