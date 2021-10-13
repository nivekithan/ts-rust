#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Float,
    String,
    Boolean,
    Unknown,
    ArrayType { base_type: Box<DataType> },
}
