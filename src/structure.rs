use hashbrown::HashMap;
mod test;
mod serde_impl;
#[derive(Debug,PartialEq)]
pub struct Wire<'w>{
    pub id:   usize,
    pub name: &'w str,
    pub size: usize,
    pub refs: &'w str,
    pub modules_list: Vec<usize>,
}
impl<'w> Wire<'w>{
    pub fn new(
            id:   usize,
            name: &'w str,
            size: usize,
            refs: &'w str,
            modules_list: Vec<usize>,
    )-> Wire<'w>{
        Wire{id,name,size,refs,modules_list}
    }
    pub fn to_json_string(&self)->String{
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json_str(json:&str)->Wire{
        serde_json::from_str::<Wire>(json).unwrap()
    }
    pub fn to_ron_string(&self)->String{
        // serde_json::to_string(self).unwrap()
        ron::to_string(&self).unwrap()
    }
    pub fn from_ron_str(ron:&str)->Wire{
        // serde_json::from_str::<Wire>(json).unwrap()
        ron::from_str(ron).unwrap()
    }
}



#[derive(Debug,PartialEq)]
pub struct Module<'m>{
    pub id:         usize,
    pub name:       &'m str,
    pub full_path:  Vec<usize>,
    pub submodules: Vec<usize>,
    pub wires_list: Vec<usize>,
}
impl<'m> Module<'m>{
    pub fn new(
        id:         usize,
        name:       &'m str,
        full_path:  Vec<usize>,
        submodules: Vec<usize>,
        wires_list: Vec<usize>,
    )-> Module<'m>{
        Module{id,name,full_path,submodules,wires_list}
    }
    pub fn to_json_string(&self)->String{
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json_str(json:&str)->Module{
        serde_json::from_str::<Module>(json).unwrap()
    }
    pub fn to_ron_string(&self)->String{
        ron::to_string(&self).unwrap()
    }
    pub fn from_ron_str(ron:&str)->Module{
        ron::from_str(ron).unwrap()
    }
}




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
// use std::io::Write;
#[derive(Debug,PartialEq)]
pub struct Structure<'w,'m,'d>{
    pub filename:    &'d str,
    pub version:     &'d str,
    pub date:        &'d str,
    pub timescale:   &'d str,
    pub all_wires:   Vec<Wire<'w>>,
    pub all_modules: Vec<Module<'m>>,
}
impl<'w,'m,'d> Structure<'w,'m,'d>{
    pub fn new(
        filename:    &'d str,
        version:     &'d str,
        date:        &'d str,
        timescale:   &'d str,
        all_wires:   Vec<Wire<'w>>,
        all_modules: Vec<Module<'m>>,
    )->Structure<'w,'m,'d>{
        Structure{
            filename,
            version,
            date,
            timescale,
            all_wires,
            all_modules,
        }
    }
    pub fn new_constructor()-> Constructor<'w,'m,'d>{
        Constructor::init()
    }
    pub fn from_constructor(con:Constructor<'w,'m,'d>)-> Structure<'w,'m,'d>{
        Structure{
            filename:    con.filename,
            version:     con.version,
            date:        con.date,
            timescale:   con.timescale,
            all_wires:   con.all_wires,
            all_modules: con.all_modules,
        }
    }
    
    pub fn to_json_string(&self)->String{
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json_str(json:&str)->Structure{
        serde_json::from_str::<Structure>(json).unwrap()
    }
    pub fn to_ron_string(&self)->String{
        ron::to_string(&self).unwrap()
    }
    pub fn from_ron_str(ron:&str)->Structure{
        ron::from_str(ron).unwrap()
    }
    // pub fn to_ron_writer(&self,writer:Write)->ron::Result<()>{
    //     ron::ser::to_writer(self)
    // }
}




