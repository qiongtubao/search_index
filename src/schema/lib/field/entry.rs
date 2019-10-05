use crate::schema::lib::field::options::int::IntOptions;
use crate::schema::lib::field::r#type::FieldType;
use crate::schema::lib::field::options::text::TextOptions;
use serde::{Serialize, Serializer, Deserializer, Deserialize};
use serde::ser::SerializeStruct;
use serde::de::{self, Visitor, MapAccess};
use std::fmt;

#[derive(Debug)]
pub struct FieldEntry {
    name: String,
    field_type: FieldType,
}

impl FieldEntry {
    pub fn new_u64(field_name: String, field_type: IntOptions) -> FieldEntry {
        FieldEntry {
            name: field_name,
            field_type: FieldType::U64(field_type),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new_text(field_name: String, field_type: TextOptions) -> FieldEntry {
        FieldEntry {
            name: field_name,
            field_type: FieldType::Str(field_type)
        }
    }
    //是否建立索引
    pub fn is_indexed(&self) -> bool {
        match self.field_type {
            FieldType::Str(ref options) => options.get_indexing_options().is_some(),
            FieldType::U64(ref options)
            | FieldType::I64(ref options)
            | FieldType::F64(ref options)
            | FieldType::Date(ref options) => options.is_indexed(),
            FieldType::HierarchicalFacet => true,
            FieldType::Bytes => false,
        }
    }
}
/**
    fieldEntry -> Object
*/
impl Serialize for FieldEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        //SerializeStruct对象 => Object
        let mut s = serializer.serialize_struct("field_entry", 3)?;
        s.serialize_field("name", &self.name)?;

        match self.field_type {
            FieldType::Str(ref options) => {
                s.serialize_field("type", "text")?;
                s.serialize_field("options", options)?;
            }
            FieldType::U64(ref options) => {
                s.serialize_field("type", "u64")?;
                s.serialize_field("options", options)?;
            }
            FieldType::I64(ref options) => {
                s.serialize_field("type", "i64")?;
                s.serialize_field("options", options)?;
            }
            FieldType::F64(ref options) => {
                s.serialize_field("type", "f64")?;
                s.serialize_field("options", options)?;
            }
            FieldType::Date(ref options) => {
                s.serialize_field("type", "date")?;
                s.serialize_field("options", options)?;
            }
            FieldType::HierarchicalFacet => {
                s.serialize_field("type", "hierarchical_facet")?;
            }
            FieldType::Bytes => {
                s.serialize_field("type", "bytes")?;
            }
        }

        s.end()
    }
}
/**
    Object -> FieldEntry
*/
impl<'de> Deserialize<'de> for FieldEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Type,
            Options,
        };

        const FIELDS: &[&str] = &["name", "type", "options"];

        struct FieldEntryVisitor;

        impl<'de> Visitor<'de> for FieldEntryVisitor {
            type Value = FieldEntry;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct FieldEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<FieldEntry, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut name = None;
                let mut ty = None;
                let mut field_type = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if ty.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let type_string = map.next_value()?;
                            match type_string {
                                "hierarchical_facet" => {
                                    field_type = Some(FieldType::HierarchicalFacet);
                                }
                                "bytes" => {
                                    field_type = Some(FieldType::Bytes);
                                }
                                "text" | "u64" | "i64" | "f64" | "date" => {
                                    // These types require additional options to create a field_type
                                }
                                _ => panic!("unhandled type"),
                            }
                            ty = Some(type_string);
                        }
                        Field::Options => match ty {
                            None => {
                                let msg = "The `type` field must be \
                                           specified before `options`";
                                return Err(de::Error::custom(msg));
                            }
                            Some(ty) => match ty {
                                "text" => field_type = Some(FieldType::Str(map.next_value()?)),
                                "u64" => field_type = Some(FieldType::U64(map.next_value()?)),
                                "i64" => field_type = Some(FieldType::I64(map.next_value()?)),
                                "f64" => field_type = Some(FieldType::F64(map.next_value()?)),
                                "date" => field_type = Some(FieldType::Date(map.next_value()?)),
                                _ => {
                                    let msg = format!("Unrecognised type {}", ty);
                                    return Err(de::Error::custom(msg));
                                }
                            },
                        },
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                ty.ok_or_else(|| de::Error::missing_field("ty"))?;
                let field_type = field_type.ok_or_else(|| de::Error::missing_field("options"))?;

                Ok(FieldEntry { name, field_type })
            }
        }

        deserializer.deserialize_struct("field_entry", FIELDS, FieldEntryVisitor)
    }
}