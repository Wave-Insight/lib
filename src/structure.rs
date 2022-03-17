use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};
// mod test;

// /// Hello world example for Rust.
// pub trait Bincode<'de> where Self:Sized + serde::Serialize + serde::Deserialize<'de> {
//     /// Hello world example for Rust.
//     #[inline]
//     fn to_json(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
//     /// Hello world example for Rust.
//     #[inline]
//     #[must_use]
//     fn from_json(json: &'de str) -> Self {
//         serde_json::from_str::<Self>(json).unwrap()
//     }
//     /// Hello world example for Rust.
//     #[inline]
//     fn to_bincode(&self)->Vec<u8>{
//         bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
//     }
//     /// Hello world example for Rust.
//     #[inline]
//     #[must_use]
//     fn from_bincode(bincode: &'de [u8]) -> Self {
//         bincode::serde::decode_borrowed_from_slice(bincode, bincode::config::standard()).unwrap()
//     }
// }


/// Hello world example for Rust.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Wire {
    /// Hello world example for Rust.
    pub id: usize,
    /// Hello world example for Rust.
    pub name: String,
    /// Hello world example for Rust.
    pub size: usize,
    /// Hello world example for Rust.
    pub refs: [usize;2],
    /// Hello world example for Rust.
    pub modules_list: Vec<usize>,
}
// impl Bincode<'static> for Wire{}
impl Wire{
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn new(
        id: usize,
        name: &str,
        size: usize,
        refs: [usize;2],
        modules_list: Vec<usize>,
    ) -> Self{
        Self{
            id,
            name:String::from(name),
            size,
            refs,
            modules_list,
        }
    }
    /// Serialize the given data structure as a String of JSON.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    #[must_use]
    #[inline]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str::<Self>(json).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn to_bincode(&self)->Vec<u8>{
        bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn from_bincode(bincode: &[u8]) -> Self {
        bincode::serde::decode_borrowed_from_slice(bincode, bincode::config::standard()).unwrap()
    }
}
/// Hello world example for Rust.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Module{
    /// Hello world example for Rust.
    pub id: usize,
    /// Hello world example for Rust.
    pub name: String,
    /// Hello world example for Rust.
    pub full_path: Vec<usize>,
    /// Hello world example for Rust.
    pub submodules: Vec<usize>,
    /// Hello world example for Rust.
    pub wires_list: Vec<usize>,
}
impl Module{
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn new(
        id: usize,
        name: &str,
        full_path: Vec<usize>,
        submodules: Vec<usize>,
        wires_list: Vec<usize>,
    ) -> Self {
        Self {
            id,
            name:String::from(name),
            full_path,
            submodules,
            wires_list,
        }
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str::<Self>(json).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn to_bincode(&self)->Vec<u8>{
        bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn from_bincode(bincode: &[u8]) -> Self {
        bincode::serde::decode_borrowed_from_slice(bincode, bincode::config::standard()).unwrap()
    }
}
/// Hello world example for Rust.
#[derive(Debug)]
pub struct Constructor{
    /// Hello world example for Rust.
    pub structure: Structure,
    /// Hello world example for Rust.
    wires_id_map: HashMap<String, usize>,
    /// Hello world example for Rust.
    wires_num: usize,
    /// Hello world example for Rust.
    modules_num: usize,
    /// Hello world example for Rust.
    curr_module_id: usize,
}
impl Constructor {
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn init() -> Self {
        Self {
            structure: Structure::new(
                String::new(), 
                String::new(), 
                String::new(), 
                String::new(), 
                vec![], 
                vec![Module::new(0, "/", vec![0], vec![], vec![])]
            ),
            wires_id_map: HashMap::new(),
            wires_num: 0,
            modules_num: 0,
            curr_module_id: 0,
        }
    }
    /// Hello world example for Rust.
    #[inline]
    pub fn set_filename(
        &mut self,
        filename:&str
    ) {
        self.structure.filename = String::from(filename);
    }
    /// Hello world example for Rust.
    #[inline]
    pub fn set_version(
        &mut self,
        version:&str
    ) {
        self.structure.version = String::from(version);
    }
    /// Hello world example for Rust.
    #[inline]
    pub fn set_date(
        &mut self,
        date:&str
    ) {
        self.structure.date = String::from(date);
    }
    /// Hello world example for Rust.
    #[inline]
    pub fn set_timescale(
        &mut self,
        timescale:&str
    ) {
        self.structure.timescale = String::from(timescale);
    }
    /// Hello world example for Rust.
    fn curr_module(&mut self)->&mut Module{
        &mut self.structure.all_modules[self.curr_module_id]
    }
    
    /// Hello world example for Rust.
    #[inline]
    pub fn new_wire(
        &mut self,
        wire_str_id: &str,
        wire_name:   &str,
        wire_size:   usize,
        wire_refs:   [usize;2],
    ) {
        if let Some(id) = self.wires_id_map.get(wire_str_id) {
            let old_id = *id;
            self.structure.all_wires[old_id]
                .modules_list
                .push(self.curr_module_id);
            self.curr_module().wires_list.push(old_id);
            // self.structure.all_modules[self.curr_module_id]
            //     .wires_list
            //     .push(old_id);
        } else {
            let new_id = self.wires_num;
            self.wires_num += 1;
            let new_wire = Wire::new(
                new_id,
                wire_name,
                wire_size,
                wire_refs,
                vec![self.curr_module_id],
            );
            let _ = self.wires_id_map.insert(String::from(wire_str_id), new_id);
            self.structure.all_wires.insert(new_id, new_wire);
            self.curr_module().wires_list
                .push(new_id);
        }
    }
    /// insert `curr_module` into `all_modules`
    ///
    /// update `modules_num`
    ///
    /// update `curr_module`
    #[inline]
    pub fn new_module(&mut self, name: &str) {
        let new_id = self.modules_num + 1;
        self.curr_module().submodules.push(new_id);
        self.modules_num = new_id;
        let mut new_full_path = self.curr_module().full_path.clone();
        // println!("{:?}",new_full_path);
        new_full_path.push(new_id);
        // let new_full_path= new_full_path;
        // println!("{:?}",new_full_path);
        self.curr_module_id = new_id;
        // println!("{:?}",new_id);
        let new_module = Module::new(
            self.curr_module_id,
            name,
            new_full_path,
            vec![],
            vec![],
        );
        println!("{:?}",new_module);
        self.structure.all_modules.push(new_module);
        println!("{:?}",self.structure);
    }
    /// Hello world example for Rust.
    #[inline]
    pub fn end_module(&mut self) {
        let _ = self.curr_module().full_path.pop();
        match self.curr_module().full_path.last() {
            Some(id) => {
                let last_id = *id;
                self.curr_module_id = last_id;
            }
            None => panic!(
                "ERROR: Can not end from module \"{}\", it is TOP module.",
                self.curr_module().name
            ),
        }
    }
}
/// Hello world example for Rust.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Structure {
    /// Hello world example for Rust.
    pub filename:    String,
    /// Hello world example for Rust.
    pub version:     String,
    /// Hello world example for Rust.
    pub date:        String,
    /// Hello world example for Rust.
    pub timescale:   String,
    /// Hello world example for Rust.
    pub all_wires:   Vec<Wire>,
    /// Hello world example for Rust.
    pub all_modules: Vec<Module>,
}
impl Structure {
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn new(
        filename: String,
        version: String,
        date: String,
        timescale: String,
        all_wires: Vec<Wire>,
        all_modules: Vec<Module>,
    ) -> Self {
        Self {
            filename,
            version,
            date,
            timescale,
            all_wires,
            all_modules,
        }
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn new_constructor() -> Constructor {
        Constructor::init()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str::<Self>(json).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn to_bincode(&self)->Vec<u8>{
        bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
    }
    /// Hello world example for Rust.
    #[must_use]
    #[inline]
    pub fn from_bincode(bincode: &[u8]) -> Self {
        bincode::serde::decode_borrowed_from_slice(bincode, bincode::config::standard()).unwrap()
    }
}

/// Hello world example for Rust.
mod test{
    /// Hello world example for Rust.
    mod wire {
        #[test]
        fn json() {
            use super::super::Wire;
            let w = Wire::new(1, "test", 2, [0,0], vec![1, 2, 3]);
            println!("{:?}", w);
            let json = w.to_json();
            println!("{}", json);
            let w_j = Wire::from_json(&json);
            println!("{:?}", w_j);
            assert_eq!(w, w_j);
        }
        #[test]
        fn bincode() {
            use super::super::Wire;
            let w = Wire::new(1, "test", 2, [0,0], vec![1, 2, 3]);
            println!("{:?}", w);
            let bincode = w.to_bincode();
            println!("{:?}", bincode);
            let w_bincode = Wire::from_bincode(&bincode);
            println!("{:?}", w_bincode);
            assert_eq!(w, w_bincode);
        }
    }
    /// Hello world example for Rust.
    mod module {
    
        #[test]
        fn json() {
            use super::super::Module;
            let m = Module::new(1, "test", vec![1, 2], vec![], vec![1, 2, 3]);
            println!("{:?}", m);
            let json = m.to_json();
            println!("{}", json);
            let m_j = Module::from_json(&json);
            println!("{:?}", m_j);
            assert_eq!(m, m_j);
        }
    
        #[test]
        fn bincode() {
            use super::super::Module;
            let m = Module::new(1, "test", vec![1, 2], vec![], vec![1, 2, 3]);
            println!("{:?}", m);
            let bincode = m.to_bincode();
            println!("{:?}", bincode);
            let m_bincode = Module::from_bincode(&bincode);
            println!("{:?}", m_bincode);
            assert_eq!(m, m_bincode);
        }
    }
    /// Hello world example for Rust.
    mod structure {
        use super::super::Structure;
        /// Hello world example for Rust.
        #[allow(dead_code)]
        fn from_constructor() -> Structure {
            let mut constructor = Structure::new_constructor();
            constructor.set_filename("input.vcd");
            constructor.set_version("Generated by VerilatedVcd");
            constructor.set_date("Mon Mar  7 16:03:36 2022");
            constructor.set_timescale("1ps");
            // find new submodule: TOP
            constructor.new_module("TOP");
            // find new wire: clk
            {
                let wire_id   = "'";
                let wire_name = "clk";
                let wire_size = 1;
                let wire_refs = [0,0];
                constructor.new_wire(wire_id, wire_name, wire_size, wire_refs);
            }
            // find new wire: io_state
            {
                let wire_id   = "&";
                let wire_name = "io_state";
                let wire_size = 8;
                let wire_refs = [7,0];
                constructor.new_wire(wire_id, wire_name, wire_size, wire_refs);
            }
            // find new wire: reset
            {
                let wire_id   = "(";
                let wire_name = "reset";
                let wire_size = 1;
                let wire_refs = [0,0];
                constructor.new_wire(wire_id, wire_name, wire_size, wire_refs);
            }
            // find new submodule: MyTopLevel
            constructor.new_module("MyTopLevel");
            // find new wire: clk
            {
                let wire_id   = "'";
                let wire_name = "clk";
                let wire_size = 1;
                let wire_refs = [0,0];
                constructor.new_wire(wire_id, wire_name, wire_size, wire_refs);
            }
            // find new wire: counter
            {
                let wire_id   = ")";
                let wire_name = "counter";
                let wire_size = 8;
                let wire_refs = [7,0];
                constructor.new_wire(wire_id, wire_name, wire_size, wire_refs);
            }
            // find End_Module
            constructor.end_module();
            // find new submodule: MyTest
            constructor.new_module("MyTest");
            // find new wire: clk
            {
                let wire_id   = "'";
                let wire_name = "clk";
                let wire_size = 1;
                let wire_refs = [0,0];
                constructor.new_wire(wire_id, wire_name, wire_size, wire_refs);
            }
            // find End_Module
            constructor.end_module();
            // find End_Module
            constructor.end_module();
            constructor.structure
            // Structure::from_constructor(&constructor)
        }
    
        #[test]
        fn constructor() {
            let structure = from_constructor();
            println!("{:?}", structure);
        }
    
        #[test]
        fn json() {
            let structure = from_constructor();
            println!("{:?}", structure);
            let json = structure.to_json();
            println!("{:?}", json);
            let s_from_json = Structure::from_json(&json);
            println!("{:?}", s_from_json);
            assert_eq!(structure, s_from_json);
        }
    
        #[test]
        fn bincode() {
            let structure = from_constructor();
            println!("{:?}", structure);
            let bincode = structure.to_bincode();
            println!("{:?}", bincode);
            let s_from_bincode = Structure::from_bincode(&bincode);
            println!("{:?}", s_from_bincode);
            assert_eq!(structure, s_from_bincode);
        }
        // #[test]
        // fn file(){
        //     use std::io::{Write, Read}; // bring trait into scope
        //     use std::fs::{OpenOptions, File, metadata};
    
        //     let path = "./tests/bincode";
        //     let structure = from_constructor();
        //     println!("{:?}", structure);
        //     let bincode = structure.to_bincode();
        //     println!("{:?}", bincode);
    
        //     let mut file:File = OpenOptions::new()
        //         .write(true)
        //         .open(path).unwrap();
        //     file.write_all(&bincode);
        //     let mut file = File::open(path).expect("no file found");
        //     let metadata = metadata(path).expect("unable to read metadata");
        //     let mut buffer = vec![0; metadata.len() as usize];
        //     file.read(&mut buffer).expect("buffer overflow");
        //     let s_from_bincode = Structure::from_bincode(&buffer);
        //     println!("{:?}", s_from_bincode);
        //     assert_eq!(structure, s_from_bincode);
        // }
    }
    
}