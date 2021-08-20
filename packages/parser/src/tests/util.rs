use std::vec;

use ast::{
    data_type::DataType,
    expression::{BinaryOperator, Expression, UnaryOperator},
};
#[derive(Debug)]
pub(crate) struct ExpressionForm {
    pub(crate) top_string: Option<String>,
    pub(crate) top_expression: Option<Expression>,
    pub(crate) main_string: String,
    pub(crate) main_exp: Expression,
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

pub(crate) fn generate_expressions(datatype: &DataType) -> Vec<ExpressionForm> {
    let every_expression_form: Vec<fn() -> ExpressionForm> = vec![
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

    let mut valid_expression_form: Vec<ExpressionForm> = vec![];

    for generate_expression_form in every_expression_form {
        let generated_expression_form = generate_expression_form();

        if &generated_expression_form.get_data_type() == datatype {
            valid_expression_form.push(generated_expression_form);
        }
    }

    return valid_expression_form;
}
