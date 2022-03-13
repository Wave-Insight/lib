
use wave_insight_lib::structure::Structure;
fn main(){
    let mut constructor = Structure::new_constructor();
    constructor.filename  = "input.vcd";
    constructor.version   = "Generated by VerilatedVcd";
    constructor.date      = "Mon Mar  7 16:03:36 2022";
    constructor.timescale = "1ps";
    // find new submodule: TOP
    constructor.new_module("TOP");
    // find new wire: clk
    let wire_id   = "'";
    let wire_name = "clk";
    let wire_size = 1;
    let wire_refs = [0,0];
    constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
    // find new wire: io_state
    let wire_id   = "&";
    let wire_name = "io_state"; 
    let wire_size = 8;
    let wire_refs = [7,0];
    constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
    // find new wire: reset
    let wire_id   = "(";
    let wire_name = "reset"; 
    let wire_size = 1;
    let wire_refs = [0,0];
    constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
    // find new submodule: MyTopLevel
    constructor.new_module("MyTopLevel");
    // find new wire: clk
    let wire_id   = "'";
    let wire_name = "clk"; 
    let wire_size = 1;
    let wire_refs = [0,0];
    constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
    // find new wire: counter
    let wire_id   = ")";
    let wire_name = "counter"; 
    let wire_size = 8;
    let wire_refs = [7,0];
    constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
    // find End_Module
    constructor.end_module();
    // find new submodule: MyTest
    constructor.new_module("MyTest");
    // find new wire: clk
    let wire_id   = "'";
    let wire_name = "clk"; 
    let wire_size = 1;
    let wire_refs = [0,0];
    constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
    // find End_Module
    constructor.end_module();
    // find End_Module
    constructor.end_module();

    let s = Structure::from_constructor(constructor);
    println!();
    println!("{:?}",s);
    println!();
    let bincode = s.to_bincode();
    println!("{:?}",bincode);
    println!();
    let s_from_bincode = Structure::from_bincode(&bincode);
    println!("{:?}",s_from_bincode);
    println!();
    assert_eq!(s,s_from_bincode);
}
