#[derive(Debug, Clone)]
enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
enum Ast {
    Literal(i64),
    Variable(String),
    Unary {
        operation: Operation,
        operand: Box<Ast>,
    },
    Binary {
        op: Operation,
        left: Box<Ast>,
        right: Box<Ast>,
    },
    Nary {
        op: Operation,
        operands: Vec<Ast>,
    },
}

// Write a function to fold the constant in the tree
fn fold_constant(ast: Ast) -> Ast {
    let result_ast = match ast {
        Ast::Literal(l) => Ast::Literal(l),
        Ast::Variable(s) => Ast::Variable(s),
        Ast::Unary { operation, operand } => match operation {
            Operation::Add => fold_constant(*operand),
            Operation::Sub => fold_constant(Ast::Binary {
                op: Operation::Mult,
                left: Box::new(Ast::Literal(-1)),
                right: operand,
            }),
            _ => *operand,
        },
        Ast::Binary { op, left, right } => match op {
            Operation::Add => match (*left.clone(), *right.clone()) {
                (Ast::Literal(l), Ast::Literal(r)) => Ast::Literal(l + r),
                _ => Ast::Binary {
                    op: op,
                    left: Box::new(fold_constant(*left)),
                    right: Box::new(fold_constant(*right)),
                },
            },
            Operation::Sub => match (*left.clone(), *right.clone()) {
                (Ast::Literal(l), Ast::Literal(r)) => Ast::Literal(l - r),
                _ => Ast::Binary {
                    op: op,
                    left: Box::new(fold_constant(*left)),
                    right: Box::new(fold_constant(*right)),
                },
            },
            Operation::Mult => match (*left.clone(), *right.clone()) {
                (Ast::Literal(l), Ast::Literal(r)) => Ast::Literal(l * r),
                _ => Ast::Binary {
                    op: op,
                    left: Box::new(fold_constant(*left)),
                    right: Box::new(fold_constant(*right)),
                },
            },
            Operation::Div => match (*left.clone(), *right.clone()) {
                (Ast::Literal(l), Ast::Literal(r)) => Ast::Literal(l / r), //Handle zero division
                _ => Ast::Binary {
                    op: op,
                    left: Box::new(fold_constant(*left)),
                    right: Box::new(fold_constant(*right)),
                },
            },
            Operation::Mod => match (*left.clone(), *right.clone()) {
                (Ast::Literal(l), Ast::Literal(r)) => Ast::Literal(l % r),
                _ => Ast::Binary {
                    op: op,
                    left: Box::new(fold_constant(*left)),
                    right: Box::new(fold_constant(*right)),
                },
            },
        },
        Ast::Nary { op, operands } => Ast::Nary { op, operands },
    };
    result_ast
}

fn main() {
    // x * (10 + 30) ->   x * 40
    let expr = Ast::Binary {
        op: Operation::Mult,
        left: Box::new(Ast::Variable("x".into())),
        right: Box::new(Ast::Binary {
            op: Operation::Add,
            left: Box::new(Ast::Literal(10)),
            right: Box::new(Ast::Literal(30)),
        }),
    };

    let transformed = fold_constant(expr);
    // expected value for transformed is Ast::Binary(Variable("x"), Ast::Literal(40))
    println!("{:?}", transformed);
}
