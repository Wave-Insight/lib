use hashbrown::HashMap;
use crate::structure::wire::Wire;
use crate::structure::module::Module;
#[derive(Debug)]
pub struct Constructor<'w,'m,'d>{
    pub filename:    &'d str,
    pub version:     &'d str,
    pub date:        &'d str,
    pub timescale:   &'d str,
    pub all_wires:   Vec<Wire<'w>>,
    pub all_modules: Vec<Module<'m>>,
    wires_id_map:    HashMap<&'w str,usize>,
    wires_num:       usize,
    modules_num:     usize,
    curr_module_id:  usize,
}
impl<'w,'m,'d> Constructor<'w,'m,'d>{
    pub fn init()-> Constructor<'w,'m,'d>{
        Constructor{
            filename:       "",
            version:        "",
            date:           "",
            timescale:      "",
            all_wires:      vec![],
            all_modules:    vec![Module::new(0,"/",vec![0],vec![],vec![])],
            wires_id_map:   HashMap::new(),
            wires_num:      0,
            modules_num:    0,
            curr_module_id: 0,
        }
    }
    pub fn new_wire(
        &mut self,
        wire_str_id: &'w str,
        wire_name:   &'w str,
        wire_size:   usize,
        wire_refs:   &'w str,
    ){
        match self.wires_id_map.get(wire_str_id){
            Some(id) => {
                let old_id = id.clone();
                self.all_wires[old_id].modules_list.push(self.curr_module_id);
                self.all_modules[self.curr_module_id].wires_list.push(old_id);
            },
            None => {
                let new_id = self.wires_num;
                self.wires_num += 1;
                let new_wire = Wire::new(new_id, wire_name, wire_size, wire_refs,vec![self.curr_module_id]);
                self.wires_id_map.insert(wire_str_id, new_id);
                self.all_wires.insert(new_id,new_wire);
                self.all_modules[self.curr_module_id].wires_list.push(new_id);
            }
        }
    }
    /// insert `curr_module` into `all_modules`
    /// 
    /// update `modules_num`
    /// 
    /// update `curr_module`
    pub fn new_module(
        &mut self,
        name: &'m str,
    ){
        let new_id = self.modules_num + 1;
        self.all_modules[self.curr_module_id].submodules.push(new_id);
        self.modules_num = new_id;
        let mut new_full_path = self.all_modules[self.curr_module_id].full_path.clone();
        new_full_path.push(new_id);
        self.curr_module_id = new_id;
        self.all_modules.push(
            Module::new(self.curr_module_id,name,new_full_path,vec![],vec![],)
        );
    }
    pub fn end_module(&mut self){
        self.all_modules[self.curr_module_id].full_path.pop();
        match self.all_modules[self.curr_module_id].full_path.last(){
            Some(id) => {
                let last_id = id.clone();
                self.curr_module_id=last_id;
            },
            None => panic!("ERROR: Can not end from module \"{}\", it is TOP module.",self.all_modules[self.curr_module_id].name)
        }
    }
}