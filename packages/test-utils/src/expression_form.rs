use std::vec;

use ast::{
    data_type::DataType,
    declaration::{Declaration, VariableDeclarationKind},
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast,
};
#[derive(Debug)]
pub struct ExpressionForm {
    pub top_string: Option<String>,
    pub top_expression: Option<Vec<Ast>>,
    pub main_exp: Expression,
    pub main_string: String,
}

impl ExpressionForm {
    fn new(main_string: &str, main_exp: Expression) -> ExpressionForm {
        return ExpressionForm {
            top_expression: None,
            top_string: None,
            main_exp,
            main_string: main_string.to_string(),
        };
    }

    fn get_data_type(&self) -> DataType {
        return self.main_exp.get_data_type();
    }

    pub fn generate_input(&self, exp_str: String) -> String {
        if let Some(top_str) = &self.top_string {
            return format!("{}\n {}", top_str, exp_str);
        } else {
            return exp_str;
        }
    }

    pub fn generate_expected_output(&self, exp_output: Vec<Ast>) -> Vec<Ast> {
        if let Some(top_exp) = &self.top_expression {
            let mut expected_output: Vec<Ast> = vec![];

            for ast in top_exp {
                let owned_ast = (*ast).clone();
                expected_output.push(owned_ast);
            }

            for ast in exp_output {
                expected_output.push(ast);
            }

            return expected_output;
        } else {
            return exp_output;
        }
    }
}

fn generate_float_literal() -> ExpressionForm {
    return ExpressionForm::new(
        "(1)",
        Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        },
    );
}

fn generate_string_literal() -> ExpressionForm {
    return ExpressionForm::new(
        "(\"1\")",
        Expression::StringLiteralExp {
            value: "1".to_string(),
        },
    );
}

fn generate_boolean_true_literal() -> ExpressionForm {
    return ExpressionForm::new(
        "(true)",
        Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        },
    );
}

fn generate_boolean_false_literal() -> ExpressionForm {
    return ExpressionForm::new(
        "(false)",
        Expression::BooleanLiteralExp {
            name: "false".to_string(),
            value: false,
        },
    );
}

fn generate_float_ident(var_name: &str) -> ExpressionForm {
    return ExpressionForm {
        top_string: Some(format!("const {} = 1;", var_name)),
        top_expression: Some(vec![Ast::Declaration(Declaration::VariableDeclaration {
            kind: VariableDeclarationKind::Const,
            ident_name: var_name.to_string(),
            exp: Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            },
        })]),
        main_string: format!("({})", var_name),
        main_exp: Expression::IdentExp {
            data_type: DataType::Float,
            name: var_name.to_string(),
        },
    };
}

fn generate_bool_true_ident(var_name: &str) -> ExpressionForm {
    return ExpressionForm {
        top_string: Some(format!("const {} = true;", var_name)),
        top_expression: Some(vec![Ast::Declaration(Declaration::VariableDeclaration {
            kind: VariableDeclarationKind::Const,
            ident_name: var_name.to_string(),
            exp: Expression::BooleanLiteralExp {
                name: "true".to_string(),
                value: true,
            },
        })]),
        main_string: format!("({})", var_name),
        main_exp: Expression::IdentExp {
            data_type: DataType::Boolean,
            name: var_name.to_string(),
        },
    };
}

fn generate_bool_false_ident(var_name: &str) -> ExpressionForm {
    return ExpressionForm {
        top_string: Some(format!("const {} = false;", var_name)),
        top_expression: Some(vec![Ast::Declaration(Declaration::VariableDeclaration {
            kind: VariableDeclarationKind::Const,
            ident_name: var_name.to_string(),
            exp: Expression::BooleanLiteralExp {
                name: "false".to_string(),
                value: false,
            },
        })]),
        main_string: format!("({})", var_name),
        main_exp: Expression::IdentExp {
            data_type: DataType::Boolean,
            name: var_name.to_string(),
        },
    };
}

fn generate_string_ident(var_name: &str) -> ExpressionForm {
    return ExpressionForm {
        top_string: Some(format!("const {} = \"1\";", var_name)),
        top_expression: Some(vec![Ast::Declaration(Declaration::VariableDeclaration {
            kind: VariableDeclarationKind::Const,
            ident_name: var_name.to_string(),
            exp: Expression::StringLiteralExp {
                value: "1".to_string(),
            },
        })]),

        main_string: format!("({})", var_name),
        main_exp: Expression::IdentExp {
            data_type: DataType::String,
            name: var_name.to_string(),
        },
    };
}

fn generate_unary_plus() -> ExpressionForm {
    return ExpressionForm::new(
        "(+1)",
        Expression::UnaryExp {
            operator: UnaryOperator::Plus,
            argument: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_unary_minus() -> ExpressionForm {
    return ExpressionForm::new(
        "(-1)",
        Expression::UnaryExp {
            operator: UnaryOperator::Minus,
            argument: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_unary_bang() -> ExpressionForm {
    return ExpressionForm::new(
        "(!true)",
        Expression::UnaryExp {
            operator: UnaryOperator::Bang,
            argument: Box::new(Expression::BooleanLiteralExp {
                name: "true".to_string(),
                value: true,
            }),
        },
    );
}

fn generate_binary_plus_float() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 + 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::Plus,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_binary_plus_string() -> ExpressionForm {
    return ExpressionForm::new(
        "(\"1\" + \"1\")",
        Expression::BinaryExp {
            operator: BinaryOperator::Plus,
            left: Box::new(Expression::StringLiteralExp {
                value: "1".to_string(),
            }),
            right: Box::new(Expression::StringLiteralExp {
                value: "1".to_string(),
            }),
        },
    );
}

fn generate_binary_minus() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 - 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::Minus,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_binary_star() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 * 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::Star,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_binary_slash() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 / 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::Slash,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_binary_vertical_bar() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 | 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::VerticalBar,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_binary_caret() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 ^ 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::Caret,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

fn generate_binary_ampersand() -> ExpressionForm {
    return ExpressionForm::new(
        "(1 & 1)",
        Expression::BinaryExp {
            operator: BinaryOperator::Ampersand,
            left: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
            right: Box::new(Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            }),
        },
    );
}

pub fn generate_expressions(datatype: &DataType, var_name: &str) -> Vec<ExpressionForm> {
    return generate_expression_filter(|cur_data_type| cur_data_type == datatype, var_name);
}

pub fn generate_expression_filter<Filter>(filter: Filter, var_name: &str) -> Vec<ExpressionForm>
where
    Filter: Fn(&DataType) -> bool,
{
    let every_0_arg_exp_form: Vec<fn() -> ExpressionForm> = vec![
        generate_float_literal,
        generate_string_literal,
        generate_boolean_true_literal,
        generate_boolean_false_literal,
        generate_unary_plus,
        generate_unary_minus,
        generate_unary_bang,
        generate_binary_plus_float,
        generate_binary_plus_string,
        generate_binary_minus,
        generate_binary_star,
        generate_binary_slash,
        generate_binary_vertical_bar,
        generate_binary_caret,
        generate_binary_ampersand,
    ];

    let every_1_arg_exp_form: Vec<fn(name: &str) -> ExpressionForm> = vec![
        generate_float_ident,
        generate_bool_true_ident,
        generate_bool_false_ident,
        generate_string_ident,
    ];

    let mut valid_expression_form: Vec<ExpressionForm> = vec![];

    for generate_expression_form in every_0_arg_exp_form {
        let generated_expression_form = generate_expression_form();

        if filter(&generated_expression_form.get_data_type()) {
            valid_expression_form.push(generated_expression_form);
        }
    }

    for generate_expression_form in every_1_arg_exp_form {
        let generated_expression_form = generate_expression_form(var_name);

        if filter(&generated_expression_form.get_data_type()) {
            valid_expression_form.push(generated_expression_form);
        }
    }

    return valid_expression_form;
}
