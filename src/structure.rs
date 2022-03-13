use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};
mod test;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Wire<'w> {
    pub id: usize,
    pub name: &'w str,
    pub size: usize,
    pub refs: [usize;2],
    pub modules_list: Vec<usize>,
}
impl<'w> Wire<'w> {
    pub fn new(
        id: usize,
        name: &'w str,
        size: usize,
        refs: [usize;2],
        modules_list: Vec<usize>,
    ) -> Wire<'w> {
        Wire {
            id,
            name,
            size,
            refs,
            modules_list,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: &str) -> Wire {
        serde_json::from_str::<Wire>(json).unwrap()
    }
    pub fn to_bincode(&self)->Vec<u8>{
        bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
    }
    pub fn from_bincode(bincode: &'w Vec<u8>) -> Wire<'w> {
        bincode::serde::decode_borrowed_from_slice(&bincode[..], bincode::config::standard()).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Module<'m> {
    pub id: usize,
    pub name: &'m str,
    pub full_path: Vec<usize>,
    pub submodules: Vec<usize>,
    pub wires_list: Vec<usize>,
}
impl<'m> Module<'m> {
    pub fn new(
        id: usize,
        name: &'m str,
        full_path: Vec<usize>,
        submodules: Vec<usize>,
        wires_list: Vec<usize>,
    ) -> Module<'m> {
        Module {
            id,
            name,
            full_path,
            submodules,
            wires_list,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: &str) -> Module {
        serde_json::from_str::<Module>(json).unwrap()
    }
    pub fn to_bincode(&self)->Vec<u8>{
        bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
    }
    pub fn from_bincode(bincode: &'m Vec<u8>) -> Module<'m> {
        bincode::serde::decode_borrowed_from_slice(&bincode[..], bincode::config::standard()).unwrap()
    }
}

#[derive(Debug)]
pub struct Constructor<'w, 'm, 'd> {
    pub filename: &'d str,
    pub version: &'d str,
    pub date: &'d str,
    pub timescale: &'d str,
    pub all_wires: Vec<Wire<'w>>,
    pub all_modules: Vec<Module<'m>>,
    wires_id_map: HashMap<&'w str, usize>,
    wires_num: usize,
    modules_num: usize,
    curr_module_id: usize,
}
impl<'w, 'm, 'd> Constructor<'w, 'm, 'd> {
    pub fn init() -> Constructor<'w, 'm, 'd> {
        Constructor {
            filename: "",
            version: "",
            date: "",
            timescale: "",
            all_wires: vec![],
            all_modules: vec![Module::new(0, "/", vec![0], vec![], vec![])],
            wires_id_map: HashMap::new(),
            wires_num: 0,
            modules_num: 0,
            curr_module_id: 0,
        }
    }
    pub fn new_wire(
        &mut self,
        wire_str_id: &'w str,
        wire_name: &'w str,
        wire_size: usize,
        wire_refs: [usize;2],
    ) {
        match self.wires_id_map.get(wire_str_id) {
            Some(id) => {
                let old_id = id.clone();
                self.all_wires[old_id]
                    .modules_list
                    .push(self.curr_module_id);
                self.all_modules[self.curr_module_id]
                    .wires_list
                    .push(old_id);
            }
            None => {
                let new_id = self.wires_num;
                self.wires_num += 1;
                let new_wire = Wire::new(
                    new_id,
                    wire_name,
                    wire_size,
                    wire_refs,
                    vec![self.curr_module_id],
                );
                self.wires_id_map.insert(wire_str_id, new_id);
                self.all_wires.insert(new_id, new_wire);
                self.all_modules[self.curr_module_id]
                    .wires_list
                    .push(new_id);
            }
        }
    }
    /// insert `curr_module` into `all_modules`
    ///
    /// update `modules_num`
    ///
    /// update `curr_module`
    pub fn new_module(&mut self, name: &'m str) {
        let new_id = self.modules_num + 1;
        self.all_modules[self.curr_module_id]
            .submodules
            .push(new_id);
        self.modules_num = new_id;
        let mut new_full_path = self.all_modules[self.curr_module_id].full_path.clone();
        new_full_path.push(new_id);
        self.curr_module_id = new_id;
        self.all_modules.push(Module::new(
            self.curr_module_id,
            name,
            new_full_path,
            vec![],
            vec![],
        ));
    }
    pub fn end_module(&mut self) {
        self.all_modules[self.curr_module_id].full_path.pop();
        match self.all_modules[self.curr_module_id].full_path.last() {
            Some(id) => {
                let last_id = id.clone();
                self.curr_module_id = last_id;
            }
            None => panic!(
                "ERROR: Can not end from module \"{}\", it is TOP module.",
                self.all_modules[self.curr_module_id].name
            ),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Structure<'w, 'm, 'd> {
    pub filename: &'d str,
    pub version: &'d str,
    pub date: &'d str,
    pub timescale: &'d str,
    #[serde(borrow)]
    pub all_wires: Vec<Wire<'w>>,
    #[serde(borrow)]
    pub all_modules: Vec<Module<'m>>,
}
impl<'w, 'm, 'd> Structure<'w, 'm, 'd> {
    pub fn new(
        filename: &'d str,
        version: &'d str,
        date: &'d str,
        timescale: &'d str,
        all_wires: Vec<Wire<'w>>,
        all_modules: Vec<Module<'m>>,
    ) -> Structure<'w, 'm, 'd> {
        Structure {
            filename,
            version,
            date,
            timescale,
            all_wires,
            all_modules,
        }
    }
    pub fn new_constructor() -> Constructor<'w, 'm, 'd> {
        Constructor::init()
    }
    pub fn from_constructor(con: Constructor<'w, 'm, 'd>) -> Structure<'w, 'm, 'd> {
        Structure {
            filename: con.filename,
            version: con.version,
            date: con.date,
            timescale: con.timescale,
            all_wires: con.all_wires,
            all_modules: con.all_modules,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: &str) -> Structure {
        serde_json::from_str::<Structure>(json).unwrap()
    }
    pub fn to_bincode(&self)->Vec<u8>{
        bincode::serde::encode_to_vec(&self, bincode::config::standard()).unwrap()
    }
    pub fn from_bincode(bincode: &'w Vec<u8>) -> Structure<'w,'w,'w> {
        bincode::serde::decode_borrowed_from_slice(&bincode[..], bincode::config::standard()).unwrap()
    }
}
