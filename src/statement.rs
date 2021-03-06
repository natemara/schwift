use crate::expression::Expression;

use std::{
    cmp,
    fs::File,
    io::{self, prelude::*},
};

#[derive(Debug, Clone)]
pub struct Statement {
    start: usize,
    end: usize,
    pub kind: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Assignment(String, Expression),
    Delete(String),
    Print(Expression),
    PrintNoNl(Expression),
    ListNew(String),
    ListAppend(String, Expression),
    ListAssign(String, Expression, Expression),
    ListDelete(String, Expression),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    While(Expression, Vec<Statement>),
    Input(String),
    Catch(Vec<Statement>, Vec<Statement>),
    Function(String, Vec<String>, Vec<Statement>),
    Return(Expression),
    FunctionCall(String, Vec<Expression>),
    DylibLoad(String, Vec<Statement>),
}

#[cfg(test)]
impl StatementKind {
    pub fn assignment<S, E>(name: S, expr: E) -> Self
    where
        S: Into<String>,
        E: Into<Expression>,
    {
        StatementKind::Assignment(name.into(), expr.into())
    }

    pub fn dylib_load<S, E>(lib_path: S, functions: Vec<Statement>) -> Self
    where
        S: Into<String>,
    {
        StatementKind::DylibLoad(lib_path.into(), functions)
    }

    pub fn return_it<E>(expr: E) -> Self
    where
        E: Into<Expression>,
    {
        StatementKind::Return(expr.into())
    }

    pub fn list_append<S, E>(name: S, expr: E) -> Self
    where
        S: Into<String>,
        E: Into<Expression>,
    {
        StatementKind::ListAppend(name.into(), expr.into())
    }

    pub fn list_delete<S, E>(name: S, expr: E) -> Self
    where
        S: Into<String>,
        E: Into<Expression>,
    {
        StatementKind::ListDelete(name.into(), expr.into())
    }

    pub fn if_block<E>(
        condition: E,
        if_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    ) -> Self
    where
        E: Into<Expression>,
    {
        StatementKind::If(condition.into(), if_body, else_body)
    }

    pub fn while_block<E>(condition: E, body: Vec<Statement>) -> Self
    where
        E: Into<Expression>,
    {
        StatementKind::While(condition.into(), body)
    }

    pub fn function<Name, Args, Body>(name: Name, args: Vec<Args>, body: Vec<Body>) -> Self
    where
        Name: Into<String>,
        Args: Into<String>,
        Body: Into<Statement>,
    {
        StatementKind::Function(
            name.into(),
            args.into_iter().map(|x| x.into()).collect(),
            body.into_iter().map(|x| x.into()).collect(),
        )
    }

    pub fn input<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        StatementKind::Input(name.into())
    }

    pub fn catch(try_block: Vec<Statement>, catch: Vec<Statement>) -> Self {
        StatementKind::Catch(try_block, catch)
    }

    pub fn list_assign<S, E, R>(name: S, index: E, assign: R) -> Self
    where
        S: Into<String>,
        E: Into<Expression>,
        R: Into<Expression>,
    {
        StatementKind::ListAssign(name.into(), index.into(), assign.into())
    }

    pub fn delete<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        StatementKind::Delete(name.into())
    }

    pub fn print<E>(expr: E) -> Self
    where
        E: Into<Expression>,
    {
        StatementKind::Print(expr.into())
    }

    pub fn new_list<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        StatementKind::ListNew(name.into())
    }
}

impl Statement {
    pub fn new(kind: StatementKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }

    #[cfg(test)]
    pub fn tnew(kind: StatementKind) -> Self {
        Self {
            kind,
            start: 0,
            end: 0,
        }
    }

    pub fn get_source(&self, filename: &str) -> io::Result<String> {
        let mut source = String::new();
        let mut f = File::open(filename)?;
        f.read_to_string(&mut source)?;

        assert!(self.start < self.end);

        Ok(source[self.start..self.end].to_string())
    }
}

#[cfg(test)]
impl cmp::PartialEq<StatementKind> for Statement {
    fn eq(&self, kind: &StatementKind) -> bool {
        self.kind == *kind
    }
}

impl cmp::PartialEq<Statement> for Statement {
    #[cfg(test)]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }

    #[cfg(not(test))]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.start == other.start && self.end == other.end
    }
}
