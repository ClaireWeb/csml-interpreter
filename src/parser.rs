pub mod csml_rules;
pub mod operator;
pub mod parse_actions;
pub mod parse_braces;
pub mod parse_comments;
pub mod parse_foreach;
pub mod parse_functions;
pub mod parse_goto;
pub mod parse_idents;
pub mod parse_if;
pub mod parse_import;
pub mod parse_literal;
pub mod parse_object;
pub mod parse_parenthesis;
pub mod parse_path;
pub mod parse_scope;
pub mod parse_string;
pub mod parse_var_types;
pub mod state_context;
pub mod tools;

use crate::parser::parse_idents::parse_idents_assignation_without_path;
pub use state_context::{ExitCondition, State, StateContext};

use crate::data::{ast::*, tokens::*};
use crate::error_format::{CustomError, ErrorInfo};
use parse_comments::comment;
use parse_scope::parse_root;
use tools::*;

use nom::error::{ErrorKind, ParseError};
use nom::{branch::alt, bytes::complete::tag, multi::fold_many0, sequence::preceded, Err, *};
use std::collections::HashMap;

fn create_flow_from_instructions(
    instructions: Vec<Instruction>,
    flow_type: FlowType,
) -> Result<Flow, String> {
    let mut elem = instructions.iter();

    // TODO: see if it can be checked in parsing
    while let Some(val) = elem.next() {
        let elem2 = elem.clone();
        for val2 in elem2 {
            if val.instruction_type == val2.instruction_type {
                return Err("StepDuplicateError".to_owned());
            }
        }
    }

    Ok(Flow {
        flow_instructions: instructions
            .into_iter()
            .map(|elem| (elem.instruction_type, elem.actions))
            .collect::<HashMap<InstructionType, Expr>>(),
        flow_type,
    })
}

pub struct Parser;

impl Parser {
    pub fn parse_flow<'a>(slice: &'a str) -> Result<Flow, ErrorInfo> {
        match start_parsing::<CustomError<Span<'a>>>(Span::new(slice)) {
            Ok((s, (instructions, flow_type))) => {
                match create_flow_from_instructions(instructions, flow_type) {
                    Ok(val) => Ok(val),
                    Err(error) => Err({
                        ErrorInfo {
                            message: error,
                            interval: Interval {
                                line: s.location_line(),
                                column: s.get_column() as u32,
                            },
                        }
                    }),
                }
            }
            Err(e) => match e {
                Err::Error(err) | Err::Failure(err) => {
                    StateContext::clear_state();
                    StateContext::clear_index();
                    Err(ErrorInfo {
                        message: err.error.to_owned(),
                        interval: Interval {
                            line: err.input.location_line(),
                            column: err.input.get_column() as u32,
                        },
                    })
                }
                Err::Incomplete(_err) => unimplemented!(),
            },
        }
    }
}

fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Instruction, E> {
    let (s, ident) = preceded(comment, parse_idents_assignation_without_path)(s)?;
    let (s, _) = preceded(comment, tag(COLON))(s)?;

    StateContext::clear_index();

    let (s, start) = get_interval(s)?;
    let (s, actions) = preceded(comment, parse_root)(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        Instruction {
            instruction_type: InstructionType::NormalStep(ident.ident),
            actions: Expr::Scope {
                block_type: BlockType::Step,
                scope: actions,
                range: RangeInterval { start, end },
            },
        },
    ))
}

fn start_parsing<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Vec<Instruction>, FlowType), E> {
    // TODO: handle FlowType::Recursive with Context
    let flow_type = FlowType::Normal;

    let (s, flow) = fold_many0(parse_step, Vec::new(), |mut acc, item| {
        acc.push(item);
        acc
    })(s)?;

    let (last, _) = comment(s)?;
    if !last.fragment().is_empty() {
        let res: IResult<Span<'a>, Span<'a>, E> =
            preceded(comment, alt((tag("ask"), tag("response"))))(last);

        let error = match res {
            Ok(_) => E::add_context(
                last,
                "use the new keyword hold to ask for user input https://docs.csml.dev/#hold",
                E::from_error_kind(last, ErrorKind::Tag),
            ),
            _ => E::from_error_kind(last, ErrorKind::Tag),
        };
        Err(Err::Failure(error))
    } else {
        Ok((s, (flow, flow_type)))
    }
}
