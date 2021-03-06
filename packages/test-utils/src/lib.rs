mod clock;

use std::collections::HashMap;

use ast::{
    data_type::DataType,
    declaration::VariableDeclarationKind,
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast, AstPtr,
};
use indexmap::indexmap;

use self::clock::Clock;

pub enum DatatypeOrFn {
    DataType(DataType),
    Fn(Box<dyn Fn(&DataType) -> bool>),
}

pub struct ExpressionTest {
    pub generate_input: Box<dyn Fn(Vec<String>, Vec<String>) -> String>, // Input we are testing
    pub expressions_data_type: Vec<(DatatypeOrFn, Vec<String>)>, // Can be used to choose what data_type of expression we have to provide

    pub test: Box<dyn Fn(Vec<Expression>, Vec<&AstPtr>, String) -> ()>, // This is where we can test
}

impl ExpressionTest {
    pub fn test(&self) {
        let t_exps: Vec<Vec<TExp>> = self
            .expressions_data_type
            .iter()
            .map(|(datatype_or_fn, var_names)| {
                return Self::get_t_exp(datatype_or_fn, var_names.clone());
            })
            .collect();

        let clock_limiter: Vec<usize> = t_exps.iter().map(|t| return t.len()).collect();

        let mut clock = Clock::new(clock_limiter);

        while clock.get_cur_time() != None {
            let time = clock.get_cur_time().unwrap();

            let valid_t_exp: Vec<&TExp> = time
                .iter()
                .enumerate()
                .map(|(index, pos)| {
                    return &t_exps[index][*pos];
                })
                .collect();

            let mut test_exp: Vec<Expression> = vec![];
            let mut test_ast: Vec<&AstPtr> = vec![];
            let mut ast_strings: Vec<String> = vec![];
            let mut test_strings: Vec<String> = vec![];

            for t_exp in valid_t_exp {
                test_exp.push(t_exp.exp.clone());

                for ast in &t_exp.asts {
                    test_ast.push(ast);
                }

                ast_strings.push(t_exp.ast_str.clone());

                test_strings.push(t_exp.exp_str.clone());
            }

            let input = (self.generate_input)(ast_strings, test_strings);

            (self.test)(test_exp, test_ast, input);

            clock.increase();
        }
    }

    fn get_t_exp(datatype_or_fn: &DatatypeOrFn, var_names: Vec<String>) -> Vec<TExp> {
        let every_0_arg_gen = [
            generate_float_literal,
            generate_boolean_true_literal,
            generate_boolean_false_literal,
            generate_string_literal,
            generate_array_float_literal,
            generate_unary_bang,
            generate_unary_minus,
            generate_unary_plus,
            generate_binary_float_plus,
            generate_binary_float_minus,
            generate_binary_float_star,
            generate_binary_float_slash,
            generate_binary_strict_equality_bool,
            generate_binary_strict_equality_float,
            generate_binary_strict_not_equal_bool,
            generate_binary_strict_not_equal_float,
            generate_binary_less_than,
            generate_binary_less_than_or_equal,
            generate_binary_greater_than,
            generate_binary_greater_than_or_equal,
            generate_object_literal,
        ];

        let every_1_arg_gen = [
            generate_boolean_ident,
            generate_float_ident,
            generate_array_float_ident,
            generate_string_ident,
        ];

        let mut valid_t_exp: Vec<TExp> = vec![];

        for arg_gen in every_0_arg_gen {
            let t_exp = arg_gen();

            if Self::is_valid(datatype_or_fn, &t_exp.exp.get_data_type()) {
                valid_t_exp.push(t_exp);
            }
        }

        for arg_gen in every_1_arg_gen {
            let t_exp = arg_gen(var_names[0].as_str());

            if Self::is_valid(datatype_or_fn, &t_exp.exp.get_data_type()) {
                valid_t_exp.push(t_exp);
            }
        }

        return valid_t_exp;
    }

    fn is_valid(datatype_or_fn: &DatatypeOrFn, datatype: &DataType) -> bool {
        match datatype_or_fn {
            DatatypeOrFn::DataType(ty) => return ty == datatype,

            DatatypeOrFn::Fn(f) => return f(datatype),
        }
    }
}

struct TExp {
    exp: Expression,
    exp_str: String,

    ast_str: String,
    asts: Vec<AstPtr>,
}

fn generate_boolean_true_literal() -> TExp {
    let exp_str = "(true)".to_string();

    let exp = Expression::BooleanLiteralExp {
        name: "true".to_string(),
        value: true,
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_boolean_false_literal() -> TExp {
    let exp_str = "(false)".to_string();

    let exp = Expression::BooleanLiteralExp {
        name: "false".to_string(),
        value: false,
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_boolean_ident(var_name: &str) -> TExp {
    let exp_str = format!("({})", var_name);

    let exp = Expression::IdentExp {
        name: var_name.to_string(),
        data_type: DataType::Boolean,
    };

    let ast_str = format!("const {} = true\n", var_name);

    let asts = vec![Ast::new_variable_declaration(
        var_name,
        Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        },
        VariableDeclarationKind::Const,
    )];

    return TExp {
        exp,
        exp_str,
        ast_str,
        asts,
    };
}

fn generate_unary_bang() -> TExp {
    let exp_str = "!(true)".to_string();

    let exp = Expression::UnaryExp {
        operator: UnaryOperator::Bang,
        argument: Box::new(Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_strict_equality_bool() -> TExp {
    let exp_str = "(true === true)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::StrictEquality,
        left: Box::new(Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        }),
        right: Box::new(Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_strict_equality_float() -> TExp {
    let exp_str = "(1 === 2)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::StrictEquality,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "2".to_string(),
            value: 2.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_strict_not_equal_bool() -> TExp {
    let exp_str = "(true !== true)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::StrictNotEqual,
        left: Box::new(Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        }),
        right: Box::new(Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_strict_not_equal_float() -> TExp {
    let exp_str = "(1 !== 2)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::StrictNotEqual,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "2".to_string(),
            value: 2.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_less_than() -> TExp {
    let exp_str = "(1 < 2)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::LessThan,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "2".to_string(),
            value: 2.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_less_than_or_equal() -> TExp {
    let exp_str = "(1 <= 2)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::LessThanOrEqual,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "2".to_string(),
            value: 2.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_greater_than() -> TExp {
    let exp_str = "(1 > 2)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::GreaterThan,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "2".to_string(),
            value: 2.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_greater_than_or_equal() -> TExp {
    let exp_str = "(1 !== 2)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::GreaterThanOrEqual,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "2".to_string(),
            value: 2.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_float_literal() -> TExp {
    let exp_str = "(1)".to_string();

    let exp = Expression::FloatLiteralExp {
        name: "1".to_string(),
        value: 1.0,
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_float_ident(var_name: &str) -> TExp {
    let exp_str = format!("({})", var_name);

    let exp = Expression::IdentExp {
        name: var_name.to_string(),
        data_type: DataType::Float,
    };

    let ast_str = format!("const {} = 1\n", var_name);

    let asts = vec![Ast::new_variable_declaration(
        var_name,
        Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        },
        VariableDeclarationKind::Const,
    )];

    return TExp {
        exp,
        exp_str,
        ast_str,
        asts,
    };
}

fn generate_unary_plus() -> TExp {
    let exp_str = "(+1)".to_string();

    let exp = Expression::UnaryExp {
        operator: UnaryOperator::Plus,
        argument: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_unary_minus() -> TExp {
    let exp_str = "(-1)".to_string();

    let exp = Expression::UnaryExp {
        operator: UnaryOperator::Minus,
        argument: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_float_plus() -> TExp {
    let exp_str = "(1 + 1)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::Plus,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_float_minus() -> TExp {
    let exp_str = "(1 - 1)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::Minus,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_float_star() -> TExp {
    let exp_str = "(1 * 1)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::Star,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_binary_float_slash() -> TExp {
    let exp_str = "(1 / 1)".to_string();

    let exp = Expression::BinaryExp {
        operator: BinaryOperator::Slash,
        left: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
        right: Box::new(Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        }),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_string_literal() -> TExp {
    let exp_str = "\"Hello World\"".to_string();

    let exp = Expression::StringLiteralExp {
        value: "Hello World".to_string(),
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_string_ident(var_name: &str) -> TExp {
    let exp_str = format!("({})", var_name);

    let exp = Expression::IdentExp {
        name: var_name.to_string(),
        data_type: DataType::String,
    };

    let ast_str = format!("const {} = \"12\"\n", var_name);

    let asts = vec![Ast::new_variable_declaration(
        var_name,
        Expression::StringLiteralExp {
            value: "12".to_string(),
        },
        VariableDeclarationKind::Const,
    )];

    return TExp {
        exp,
        exp_str,
        ast_str,
        asts,
    };
}

fn generate_array_float_literal() -> TExp {
    let exp_str = "[1, 1]".to_string();

    let exp = Expression::ArrayLiteral {
        expression: Box::new(vec![
            Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            },
            Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            },
        ]),
        expression_data_type: DataType::Float,
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}

fn generate_array_float_ident(var_name: &str) -> TExp {
    let exp_str = format!("({})", var_name);

    let exp = Expression::IdentExp {
        name: var_name.to_string(),
        data_type: DataType::ArrayType {
            base_type: Box::new(DataType::Float),
        },
    };

    let ast_str = format!("const {} = [1,1]\n", var_name);

    let asts = vec![Ast::new_variable_declaration(
        var_name,
        Expression::ArrayLiteral {
            expression: Box::new(vec![
                Expression::FloatLiteralExp {
                    name: "1".to_string(),
                    value: 1.0,
                },
                Expression::FloatLiteralExp {
                    name: "1".to_string(),
                    value: 1.0,
                },
            ]),
            expression_data_type: DataType::Float,
        },
        VariableDeclarationKind::Const,
    )];

    return TExp {
        exp,
        exp_str,
        ast_str,
        asts,
    };
}

fn generate_object_literal() -> TExp {
    let exp_str = "{a : 1, b : true}".to_string();

    let mut exp_hash_map: HashMap<String, Expression> = HashMap::new();

    exp_hash_map.insert(
        "a".to_string(),
        Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        },
    );
    exp_hash_map.insert(
        "b".to_string(),
        Expression::BooleanLiteralExp {
            name: "true".to_string(),
            value: true,
        },
    );

    let exp = Expression::ObjectLiteral {
        data_type: DataType::ObjectType {
            entries: indexmap! {"a".to_string() => DataType::Float, "b".to_string() => DataType::Boolean},
        },
        expression: exp_hash_map,
    };

    return TExp {
        exp,
        exp_str,
        ast_str: "\n".to_string(),
        asts: vec![],
    };
}
