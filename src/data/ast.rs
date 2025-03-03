use crate::data::tokens::*;
use crate::data::Literal;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Flow {
    pub flow_instructions: HashMap<InstructionType, Expr>,
    pub flow_type: FlowType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum FlowType {
    Normal,
    Recursive,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum InstructionType {
    NormalStep(String),
    //hook ?
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionType::NormalStep(ref idents) => write!(f, "{}", idents),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub actions: Expr,
}

#[derive(PartialEq, Debug, Clone)]
pub enum GotoType {
    Hook,
    Step,
    Flow,
}

#[derive(Debug, Clone)]
pub enum DoType {
    Update(Box<Expr>, Box<Expr>),
    Exec(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub interval: Interval,
    // TODO: update to Vec<Expr>
    pub args: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    Goto(GotoType, Identifier),
    Hold(Interval),
    Use(Box<Expr>),
    Do(DoType),
    Say(Box<Expr>),
    Remember(Identifier, Box<Expr>),
    Assign(Identifier, Box<Expr>),
    As(Identifier, Box<Expr>),
    Import {
        step_name: Identifier,
        as_name: Option<Identifier>,
        file_path: Option<Identifier>,
    },
    Normal(Function),
    Break(Interval),
}

#[derive(PartialEq, Debug, Clone)]
pub struct InstructionInfo {
    pub index: usize,
    pub total: usize,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub commands: Vec<(Expr, InstructionInfo)>,
    pub hooks: Vec<Hook>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Hook {
    pub index: i64,
    pub name: String,
    pub step: String,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            hooks: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum BlockType {
    LoopBlock,
    Block,
    IfLoop,
    Step,
}

#[derive(Debug, Clone)]
pub enum IfStatement {
    IfStmt {
        cond: Box<Expr>,
        consequence: Block,
        then_branch: Option<(Box<IfStatement>, InstructionInfo)>,
    },
    ElseStmt(Block, RangeInterval),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Scope {
        block_type: BlockType,
        scope: Block,
        range: RangeInterval,
    },
    ForEachExpr(
        Identifier,
        Option<Identifier>,
        Box<Expr>,
        Block,
        RangeInterval,
    ),
    ComplexLiteral(Vec<Expr>, RangeInterval),
    MapExpr(HashMap<String, Expr>, RangeInterval),
    VecExpr(Vec<Expr>, RangeInterval),
    InfixExpr(Infix, Box<Expr>, Box<Expr>), // RangeInterval ?
    ObjectExpr(ObjectType),                 // RangeInterval ?
    IfExpr(IfStatement),

    // BuilderExpr(BuilderType, Vec<Expr>),
    IdentExpr(Identifier),

    LitExpr(Literal),
}

impl Expr {
    pub fn new_idents(
        ident: String,
        interval: Interval,
        path: Option<Vec<(Interval, PathExpr)>>,
    ) -> Identifier {
        Identifier {
            ident,
            interval,
            path,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Addition,
    Subtraction,
    Divide,
    Multiply,
    Remainder,

    Not,
    Match,

    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,

    And,
    Or,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct RangeInterval {
    pub start: Interval,
    pub end: Interval,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy)]
pub struct Interval {
    pub line: u32,
    pub column: u32,
}

impl Interval {
    pub fn new(span: Span) -> Self {
        Self {
            line: span.location_line(),
            column: span.get_column() as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PathExpr {
    ExprIndex(Expr),
    StringIndex(String),
    Func(Function),
}

#[derive(Debug, Clone)]
pub enum PathLiteral {
    VecIndex(usize),
    MapIndex(String),
    Func {
        name: String,
        interval: Interval,
        // TODO: update to Vec<Literal>
        args: Literal,
    },
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub ident: String,
    pub interval: Interval,
    pub path: Option<Vec<(Interval, PathExpr)>>,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ident.partial_cmp(&other.ident)
    }
}
