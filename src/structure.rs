pub mod wire;
use wire::Wire;
pub mod module;
use module::Module;
pub mod constructor;
use constructor::Constructor;
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


use serde::{Serialize,Serializer};
// use serde::ser::{SerializeStruct, SerializeTupleStruct, SerializeStructVariant, SerializeTupleVariant};
use serde::ser::SerializeStruct;
impl<'w,'m,'d> Serialize for Structure<'w,'m,'d> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("S", 6)?;
        state.serialize_field("f", &self.filename)?;
        state.serialize_field("v", &self.version)?;
        state.serialize_field("d", &self.date)?;
        state.serialize_field("t", &self.timescale)?;
        state.serialize_field("w", &self.all_wires)?;
        state.serialize_field("m", &self.all_modules)?;
        state.end()
    }
}

use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
impl<'de:'w+'m+'d,'w,'m,'d> Deserialize<'de> for Structure<'w,'m,'d> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {Filename,Version,Date,Timescale,AllWires,AllModules}

        // 这部分也可以如下写法生成，用于解析结构体的Key
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`f`, `v`, `d`, `t`, `w` or `m`")
                    }
                    
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "f" => Ok(Field::Filename),
                            "v" => Ok(Field::Version),
                            "d" => Ok(Field::Date),
                            "t" => Ok(Field::Timescale),
                            "w" => Ok(Field::AllWires),
                            "m" => Ok(Field::AllModules),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Structure<'de,'de,'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Module")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Structure<'de,'de,'de>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let filename    = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let version     = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let date        = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let timescale   = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let all_wires   = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let all_modules = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(4, &self))?;
                Ok(Structure::new(filename,version,date,timescale,all_wires,all_modules))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Structure<'de,'de,'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut filename    = None;
                let mut version     = None;
                let mut date        = None;
                let mut timescale   = None;
                let mut all_wires   = None;
                let mut all_modules = None;
                // next_key 方法将调用 Field 类型的 deserialize 方法
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Filename => {
                            if filename.is_some() {
                                return Err(de::Error::duplicate_field("f"));
                            }
                            filename = Some(map.next_value()?);
                        }
                        Field::Version => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("v"));
                            }
                            version = Some(map.next_value()?);
                        }
                        Field::Date => {
                            if date.is_some() {
                                return Err(de::Error::duplicate_field("d"));
                            }
                            date = Some(map.next_value()?);
                        }
                        Field::Timescale => {
                            if timescale.is_some() {
                                return Err(de::Error::duplicate_field("t"));
                            }
                            timescale = Some(map.next_value()?);
                        }
                        Field::AllWires => {
                            if all_wires.is_some() {
                                return Err(de::Error::duplicate_field("w"));
                            }
                            all_wires = Some(map.next_value()?);
                        }
                        Field::AllModules => {
                            if all_modules.is_some() {
                                return Err(de::Error::duplicate_field("m"));
                            }
                            all_modules = Some(map.next_value()?);
                        }
                    }
                }
                let filename    = filename.ok_or_else(|| de::Error::missing_field("f"))?;
                let version     = version.ok_or_else(|| de::Error::missing_field("v"))?;
                let date        = date.ok_or_else(|| de::Error::missing_field("d"))?;
                let timescale   = timescale.ok_or_else(|| de::Error::missing_field("t"))?;
                let all_wires   = all_wires.ok_or_else(|| de::Error::missing_field("w"))?;
                let all_modules = all_modules.ok_or_else(|| de::Error::missing_field("m"))?;
                Ok(Structure::new(filename,version,date,timescale,all_wires,all_modules))
            }
        }
        // 调用 deserialize_struct 传递 Visitor
        const FIELDS: &'static [&'static str] = &["f","v","d","t","w","m"];
        deserializer.deserialize_struct("S", FIELDS, DurationVisitor)
    }
}

mod test{
    use super::*;
    fn structure_from_constructor()->Structure<'static, 'static, 'static>{
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
        let wire_refs = "";
        constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
        // find new wire: io_state
        let wire_id   = "&";
        let wire_name = "io_state"; 
        let wire_size = 8;
        let wire_refs = "[7:0]";
        constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
        // find new wire: reset
        let wire_id   = "(";
        let wire_name = "reset"; 
        let wire_size = 1;
        let wire_refs = "";
        constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
        // find new submodule: MyTopLevel
        constructor.new_module("MyTopLevel");
        // find new wire: clk
        let wire_id   = "'";
        let wire_name = "clk"; 
        let wire_size = 1;
        let wire_refs = "";
        constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
        // find new wire: counter
        let wire_id   = ")";
        let wire_name = "counter"; 
        let wire_size = 8;
        let wire_refs = "[7:0]";
        constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
        // find End_Module
        constructor.end_module();
        // find new submodule: MyTest
        constructor.new_module("MyTest");
        // find new wire: clk
        let wire_id   = "'";
        let wire_name = "clk"; 
        let wire_size = 1;
        let wire_refs = "";
        constructor.new_wire(wire_id,wire_name,wire_size,wire_refs);
        // find End_Module
        constructor.end_module();
        // find End_Module
        constructor.end_module();

        Structure::from_constructor(constructor)
    }
    
    #[test]
    fn from_constructor(){
        let structure = structure_from_constructor();
        println!("{:?}",structure);
    }
    
    #[test]
    fn json_str(){
        let structure = structure_from_constructor();
        println!("{:?}",structure);
        let json = structure.to_json_string();
        println!("{:?}",json);
        let s_from_json = Structure::from_json_str(&json);
        println!("{:?}",s_from_json);
        assert_eq!(structure,s_from_json)
    }
    
    #[test]
    fn ron_str(){
        let structure = structure_from_constructor();
        println!("{:?}",structure);
        let ron = structure.to_ron_string();
        println!("{:?}",ron);
        let s_from_ron = Structure::from_ron_str(&ron);
        println!("{:?}",s_from_ron);
        assert_eq!(structure,s_from_ron)
    }
}