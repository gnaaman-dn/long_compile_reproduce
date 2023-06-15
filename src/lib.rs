#[rustfmt::skip]
pub mod data3;

pub use data3::*;

// Below enums and structs should kept in sync with rust_templates/static_templates

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    Container,
    List,
    Leaf,
    LeafList,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Bool(bool),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Double(f64),
    String(&'static str),
    Empty(bool),
    Array(&'static [Value]),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnionType {
    NonNumeric,
    UInt,
    Int,
    Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    None, // For non-leaf types like container/list/etc..
    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Double,
    String,
    Empty,
    Union(UnionType),
}

#[derive(Clone, Debug)]
pub struct MetadataItem {
    pub yid: u32,
    pub node_type: NodeType,
    pub is_private: bool,
    pub is_private_restored_in_rollback: bool,

    pub value_type: ValueType,
    pub default: Value,

    // String related fields: unless specified otherwise, all strings are in python format
    pub path: &'static str, // also known as basepath, is in yang format
    pub name: &'static str,
    pub alternate_name: &'static str,
    pub children: &'static phf::Map<&'static str, u32>,
    pub key_names: &'static [&'static str], // key-names which are used for path creations, are in yang format
    pub links: &'static [&'static str],
}

pub type MdRef = &'static MetadataItem;

