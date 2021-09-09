use ast::declaration::VariableDeclarationKind;

pub struct SwitchVarDec {
    pub kind: VariableDeclarationKind,
    pub name: String,
}

pub fn generate_switch_var_dec(kind: Option<&VariableDeclarationKind>) -> Vec<SwitchVarDec> {
    let const_switch_var_dec = SwitchVarDec {
        kind: VariableDeclarationKind::Const,
        name: "const".to_string(),
    };
    let let_switch_var_dec = SwitchVarDec {
        kind: VariableDeclarationKind::Let,
        name: "let".to_string(),
    };

    if let Some(kind) = kind {
        match kind {
            VariableDeclarationKind::Const => return vec![const_switch_var_dec],
            VariableDeclarationKind::Let => return vec![let_switch_var_dec],
        }
    } else {
        return vec![const_switch_var_dec, let_switch_var_dec];
    }
}
