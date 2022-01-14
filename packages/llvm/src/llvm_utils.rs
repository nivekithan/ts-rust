use ast::data_type::DataType;
use inkwell::{
    context::Context,
    types::{
        enums::{AddressSpace, BasicTypeEnum},
        traits::BasicTypeTrait,
    },
};

pub(crate) trait LLVMUtils<'a> {
    fn force_to_basic_type(&self, context: &'a Context) -> BasicTypeEnum<'a>;
}

impl<'a> LLVMUtils<'a> for DataType {
    /*
     * Forcefully converts datatype to Basictype
     *
     *   DataType::Float => BasicType::FloatType
     *   DataType::Boolean => BasicType::Int
     *   DataType::Void => BasicType::Void
     *   DataType::FunctionType => BasicType::Pointer
     *   DataType::ObjectType :: BasicType::Pointer
     *   DataType::
     *  */
    fn force_to_basic_type(&self, context: &'a Context) -> BasicTypeEnum<'a> {
        match self {
            DataType::Float => context.f64_type().as_basic_type_enum(),
            DataType::Boolean => context.i1_type().as_basic_type_enum(),
            DataType::Void => context.void_type().as_basic_type_enum(),
            DataType::FunctionType {
                return_type,
                arguments,
            } => {
                let return_type = return_type.force_to_basic_type(context);
                let arguments: Vec<BasicTypeEnum> = arguments
                    .iter()
                    .map(|data_type| {
                        return data_type.force_to_basic_type(context);
                    })
                    .collect();

                let fn_type = return_type.fn_type(&arguments, false);
                let ptr_type = fn_type.ptr_type(AddressSpace::Generic);
                return ptr_type.as_basic_type_enum();
            }

            DataType::ObjectType { entries } => {
                let mut field_types: Vec<BasicTypeEnum> = Vec::new();

                for (_, datatype) in entries {
                    let basic_type = datatype.force_to_basic_type(context);
                    field_types.push(basic_type);
                }

                let struct_type = context.struct_type(&field_types, true);
                return struct_type
                    .ptr_type(AddressSpace::Generic)
                    .as_basic_type_enum();
            }
            DataType::String => context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .as_basic_type_enum(),

            _ => panic!("Cannot convert data_type {:?} to BasicTypeEnum", self),
        }
    }
}
