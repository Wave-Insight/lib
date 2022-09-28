//! Sample Molt Extension
//! WISH: Wave-Insight Shell 
use molt::check_args;
use molt::molt_ok;
use molt::Interp;
use molt::types::*;

use crate::AppContext;

// The AppContext

/// Install the extension's commands into the Interp.
pub fn new() ->Interp{
    Interp::new()
}

/// Install the extension's commands into the Interp.
pub fn install(interp: &mut Interp, context_id: ContextID) -> MoltResult {
    // Install a command implemented in Rust.
    interp.add_command("double", cmd_double);
    interp.add_context_command("open", cmd_open, context_id);
    interp.add_context_command("test", cmd_test, context_id);

    // NEXT, load the extension's Tcl code
    if let Err(exception) = interp.eval(include_str!("wish.tcl")) {
        panic!("Error in benchmark Tcl library: {}", exception.value().as_str());
    }

    molt_ok!()
}

/// # double *x*
///
/// Computes the double of a value
fn cmd_double(_interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    let _ = check_args(1, argv, 2, 2, "x")?;

    // Get x, if it's an integer
    let x = argv[1].as_int()?;

    // Return the result.
    molt_ok!(2 * x)
}

fn cmd_open(_interp: &mut Interp, context_id: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    let _ = check_args(1, argv, 2, 2, "value")?;
    let ctx = _interp.context::<AppContext>(context_id);

    // let file = argv[1].as_str();
    if let Ok(c) = crate::parser::vcd("/Users/junzhuo/Developer/Wave-Insight/lib/tests/input.vcd"){
        // print!("{:?}",c);
        ctx.structure = Some(c.structure);
        // ctx.structure = Result();
    };

    // Return the result.
    molt_ok!()
}

fn cmd_test(_interp: &mut Interp, context_id: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    let _ = check_args(1, argv, 1, 1, "")?;
    let ctx = _interp.context::<AppContext>(context_id);

    // let file = argv[1].as_str();
    println!("{:?}",ctx.structure);

    // Return the result.
    molt_ok!()
}