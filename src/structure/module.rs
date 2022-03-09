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
use serde::{Serialize,Serializer};
// use serde::ser::{SerializeStruct, SerializeTupleStruct, SerializeStructVariant, SerializeTupleVariant};
use serde::ser::SerializeStruct;
impl<'w> Serialize for Module<'w> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("W", 5)?;
        state.serialize_field("i", &self.id)?;
        state.serialize_field("n", &self.name)?;
        state.serialize_field("f", &self.full_path)?;
        state.serialize_field("s", &self.submodules)?;
        state.serialize_field("w", &self.wires_list)?;
        state.end()
    }
}
use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
impl<'de:'w,'w> Deserialize<'de> for Module<'w> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, Name, FullPath, Submodules, WiresList}

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
                        formatter.write_str("`i`, `n`, `f`, `s` or `w`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "i" => Ok(Field::Id),
                            "n" => Ok(Field::Name),
                            "f" => Ok(Field::FullPath),
                            "s" => Ok(Field::Submodules),
                            "w" => Ok(Field::WiresList),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Module<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Module")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Module<'de>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let full_path = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let submodules = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let wires_list = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                Ok(Module::new(id, name, full_path, submodules, wires_list))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Module<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id         = None;
                let mut name       = None;
                let mut full_path  = None;
                let mut submodules = None;
                let mut wires_list = None;
                // next_key 方法将调用 Field 类型的 deserialize 方法
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("i"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("n"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::FullPath => {
                            if full_path.is_some() {
                                return Err(de::Error::duplicate_field("f"));
                            }
                            full_path = Some(map.next_value()?);
                        }
                        Field::Submodules => {
                            if submodules.is_some() {
                                return Err(de::Error::duplicate_field("s"));
                            }
                            submodules = Some(map.next_value()?);
                        }
                        Field::WiresList => {
                            if wires_list.is_some() {
                                return Err(de::Error::duplicate_field("w"));
                            }
                            wires_list = Some(map.next_value()?);
                        }
                    }
                }
                let id         = id.ok_or_else(|| de::Error::missing_field("i"))?;
                let name       = name.ok_or_else(|| de::Error::missing_field("n"))?;
                let full_path  = full_path.ok_or_else(|| de::Error::missing_field("f"))?;
                let submodules = submodules.ok_or_else(|| de::Error::missing_field("s"))?;
                let wires_list = wires_list.ok_or_else(|| de::Error::missing_field("w"))?;
                Ok(Module::new(id, name, full_path, submodules, wires_list))
            }
        }
        // 调用 deserialize_struct 传递 Visitor
        const FIELDS: &'static [&'static str] = &["i", "n", "s", "r", "m"];
        deserializer.deserialize_struct("W", FIELDS, DurationVisitor)
    }
}

mod test{
    #[test]
    fn json_str(){
        use super::*;
        let m = Module::new(1, "test", vec![1,2], vec![], vec![1,2,3]);
        println!("{:?}",m);
        let json = m.to_json_string();
        println!("{}",json);
        let m_j = Module::from_json_str(&json);
        println!("{:?}",m_j);
        assert_eq!(m,m_j);
    }
    
    #[test]
    fn ron_str(){
        use super::*;
        let m = Module::new(1, "test", vec![1,2], vec![], vec![1,2,3]);
        println!("{:?}",m);
        let ron = m.to_ron_string();
        println!("{}",ron);
        let m_r = Module::from_ron_str(&ron);
        println!("{:?}",m_r);
        assert_eq!(m,m_r);
    }
}