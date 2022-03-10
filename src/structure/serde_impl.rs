use super::Wire;
use super::Module;
use super::Structure;
use serde::{Serialize,Serializer};
// use serde::ser::{SerializeStruct, SerializeTupleStruct, SerializeStructVariant, SerializeTupleVariant};;
use serde::ser::SerializeStruct;
use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

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