use inkwell::basic_block::BasicBlock;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeOfIfBlock {
    IfBlock,
    IfAndElseIf,
    IfAndElse,
    IfElseIfAndElse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NextElsIfBlock<'a> {
    Exit(&'a BasicBlock<'a>),
    ElseIfBlock(&'a (BasicBlock<'a>, BasicBlock<'a>)),
    Else(&'a BasicBlock<'a>),
}
