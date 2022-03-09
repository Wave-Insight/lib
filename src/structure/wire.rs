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
use serde::{Serialize,Serializer};
// use serde::ser::{SerializeStruct, SerializeTupleStruct, SerializeStructVariant, SerializeTupleVariant};
use serde::ser::SerializeStruct;
impl<'w> Serialize for Wire<'w> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("W", 5)?;
        state.serialize_field("i", &self.id)?;
        state.serialize_field("n", &self.name)?;
        state.serialize_field("s", &self.size)?;
        state.serialize_field("r", &self.refs)?;
        state.serialize_field("m", &self.modules_list)?;
        state.end()
    }
}
use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
impl<'de:'w,'w> Deserialize<'de> for Wire<'w> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, Name, Size, Refs, ModulesList}

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
                        formatter.write_str("`i`, `n`, `s`, `r` or `m`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "i" => Ok(Field::Id),
                            "n" => Ok(Field::Name),
                            "s" => Ok(Field::Size),
                            "r" => Ok(Field::Refs),
                            "m" => Ok(Field::ModulesList),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Wire<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Wire")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Wire<'de>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let size = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let refs = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let modules_list = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                Ok(Wire::new(id, name, size, refs, modules_list))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Wire<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id           = None;
                let mut name         = None;
                let mut size         = None;
                let mut refs         = None;
                let mut modules_list = None;
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
                        Field::Size => {
                            if size.is_some() {
                                return Err(de::Error::duplicate_field("s"));
                            }
                            size = Some(map.next_value()?);
                        }
                        Field::Refs => {
                            if refs.is_some() {
                                return Err(de::Error::duplicate_field("r"));
                            }
                            refs = Some(map.next_value()?);
                        }
                        Field::ModulesList => {
                            if modules_list.is_some() {
                                return Err(de::Error::duplicate_field("m"));
                            }
                            modules_list = Some(map.next_value()?);
                        }
                    }
                }
                let id           = id.ok_or_else(|| de::Error::missing_field("i"))?;
                let name         = name.ok_or_else(|| de::Error::missing_field("n"))?;
                let size         = size.ok_or_else(|| de::Error::missing_field("s"))?;
                let refs         = refs.ok_or_else(|| de::Error::missing_field("r"))?;
                let modules_list = modules_list.ok_or_else(|| de::Error::missing_field("m"))?;
                Ok(Wire::new(id, name, size, refs, modules_list))
            }
        }
        // 调用 deserialize_struct 传递 Visitor
        const FIELDS: &'static [&'static str] = &["i", "n", "s", "r", "m"];
        deserializer.deserialize_struct("W", FIELDS, DurationVisitor)
    }
}

mod test{
    use super::*;
    #[test]
    fn json_str(){
        let w = Wire::new(1, "test", 2, "none", vec![1,2,3]);
        println!("{:?}",w);
        let json = w.to_json_string();
        println!("{}",json);
        let w_j = Wire::from_json_str(&json);
        println!("{:?}",w_j);
        assert_eq!(w,w_j);
    }
    #[test]
    fn ron_str(){
        let w = Wire::new(1, "test", 2, "none", vec![1,2,3]);
        println!("{:?}",w);
        let ron = w.to_ron_string();
        println!("{}",ron);
        let w_j = Wire::from_ron_str(&ron);
        println!("{:?}",w_j);
        assert_eq!(w,w_j);
    }
}