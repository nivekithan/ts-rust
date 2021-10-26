use indexmap::IndexMap;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Float,
    String,
    Boolean,
    Unknown,
    ArrayType {
        base_type: Box<DataType>,
    },
    ObjectType {
        entries: IndexMap<String, DataType>,
    },
    FunctionType {
        return_type: Box<DataType>,
        arguments: IndexMap<String, DataType>,
    },
}
