pub mod lexer;
pub mod parser;


pub trait ASTVisitor {
    fn visit_statement(&mut self, statement: &ASTStatement);
    fn visit_expression(&mut self, expression: &ASTExpression);
    fn visit_number(&mut self, number: &i64); // Directly pass the i64 value
}

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&mut self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }
}

pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTStatement {
    pub kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement {
            kind: ASTStatementKind::Expression(expr),
        }
    }
}

pub enum ASTExpressionKind {
    Number(i64),
}

pub struct ASTExpression {
    pub kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn number(value: i64) -> Self {
        ASTExpression {
            kind: ASTExpressionKind::Number(value),
        }
    }
}

// dummy implementation of ASTVisitor
pub struct MyASTVisitor {
    indent_level: usize,
}

impl MyASTVisitor {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    fn print_indent(&self) {
        for _ in 0..self.indent_level {
            print!("  "); // Two spaces per indent level
        }
    }

    fn visit_with_indent<T>(&mut self, visit_fn: impl FnOnce(&mut Self, &T), item: &T) {
        self.indent_level += 1;
        visit_fn(self, item);
        self.indent_level -= 1;
    }
}

impl ASTVisitor for MyASTVisitor {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_indent();
        println!("Statement:");
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_with_indent(Self::visit_expression, expr),
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_indent();
        println!("Expression:");
        match &expression.kind {
            ASTExpressionKind::Number(val) => self.visit_with_indent(Self::visit_number, val),
        }
    }

    fn visit_number(&mut self, number: &i64) {
        self.print_indent();
        println!("Number: {}", number);
    }
}
