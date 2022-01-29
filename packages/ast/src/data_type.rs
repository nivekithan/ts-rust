use indexmap::IndexMap;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Float,
    String,
    Boolean,

    Void, // only permitted as a function return type

    Unknown,
    ArrayType {
        base_type: Box<DataType>,
    },
    ObjectType {
        entries: IndexMap<String, DataType>,
    },
    FunctionType {
        return_type: Box<DataType>,
        arguments: Vec<DataType>,
    },

    /*
     * If parser cannot figure out datatype for an variable it will be marked as
     * NA in Ast, it is different from Datatype::Unknown
     *
     * For example in this case
     *
     * ```ts
     * const x = foo(6);
     *
     * function foo(a : number) : number {
     *     return a;
     * }
     * ```
     *
     * When parsing statement
     * ```
     * const x = foo(6)
     * ```
     *
     * Function declaration for `foo` is below the this statement it means
     * that parser cannot know its datatype therefore it cannot find
     * datatype for `x` in this case `x` and `foo` will be assigned DataType::NA
     *
     * Once after parsing function declaration for `foo` parser will parse
     * above statement again this time it can typecheck and assign correct variable
     * for it
     *
     * llvm will expect all variables to not have DataType::NA
     *
     *  */
    NA,
}
