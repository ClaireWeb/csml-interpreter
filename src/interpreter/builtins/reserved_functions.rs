use rand::Rng;
use std::borrow::Borrow;

use crate::parser::{ast::{Expr, Literal, ReservedFunction}, tokens::*};
use crate::interpreter:: {
    message::*,
    json_to_rust::*,
    variable_handler::*,
    builtins::*,
};

pub fn remember(name: String, value: String) -> MessageType {
    MessageType::Assign{name, value}
}

pub fn typing(args: &Expr, name: String) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr{lit: Literal::IntLiteral(_)} = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err("Builtin Typing bad argument".to_owned())
    }

    Err("Builtin Typing bad argument".to_owned())
}

pub fn wait(args: &Expr, name: String) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr{lit: Literal::IntLiteral(_)} = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err("Builtin Wait bad argument".to_owned())
    }

    Err("Builtin Wait bad argument".to_owned())
}

pub fn text(args: &Expr, name: String) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr{..} = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err("Builtin Text bad argument".to_owned())
    }

    Err("Builtin Text bad argument".to_owned())
}

pub fn img(args: &Expr, name: String) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr{..} = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err("Builtin Image bad argument".to_owned())
    }

    Err("Builtin Image bad argument".to_owned())
}

pub fn url(args: &Expr, name: String) -> Result<MessageType, String>{
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr{..} = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err("Builtin Url bad argument".to_owned())
    }

    Err("Builtin Url bad argument".to_owned())
}

pub fn one_of(args: &Expr, elem_type: String, memory: &Memory, event: &Option<Event>) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        let value = &vec[rand::thread_rng().gen_range(0, vec.len())];
        let literal = get_var_from_ident(memory, event, value)?;

        return Ok(MessageType::Msg(Message::new(&Expr::LitExpr{lit: literal}, elem_type)));
    }

    Err("Builtin One_of bad argument".to_owned())
}

fn parse_quickbutton(val: String, buttton_type: String,  accepts: &mut Vec<String>) -> Button {
    accepts.push(val.clone());

    Button {
        title: val.clone(),
        buttton_type,
        accepts: vec![val.clone()],
        key: val.clone(),
        value: val.clone(),
        payload: val,
    }
}

fn match_buttons(buttons: &mut Vec<Button>, button_type: &Expr, accepts: &mut Vec<String>, name: &str, expr: &Expr, memory: &Memory, event: &Option<Event>) -> Result<bool, String> {
    match (name, expr.borrow()) {
        (BUTTON, Expr::VecExpr(expr_vec))   => {
            for elem in expr_vec.iter() {
                buttons.push(parse_quickbutton(
                    get_var_from_ident(memory, event, elem)?.to_string(),
                    get_var_from_ident(memory, event, button_type)?.to_string(),
                    accepts)
                );
            }
        }
        _                                   => return Err("bad Button Type".to_owned())
    }

    Ok(true)
}

fn parse_question(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<Question, String> {
    let expr_title = search_for_key_in_vec("title", vec)?; // Option
    let button_type = search_for_key_in_vec("button_type", vec)?; // Option
    let expr_buttons = expr_to_vec(search_for_key_in_vec("buttons", vec)?)?; // Option

    let mut buttons: Vec<Button> = vec![];
    let mut accepts: Vec<String> = vec![];

    for button in expr_buttons.iter() {
        if let Expr::FunctionExpr(ReservedFunction::Normal(name), expr) = button {
            match_buttons(&mut buttons, &button_type, &mut accepts, &name, &expr, memory, event)?;
        }
        // else { WARNING bad element }
    }

    Ok(Question {
        title: get_var_from_ident(memory, event, expr_title)?.to_string(),
        accepts,
        buttons,
    })
}

pub fn question(args: &Expr, name: String, memory: &Memory, event: &Option<Event>) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        let question = parse_question(&vec, memory, event)?;

        return Ok(MessageType::Msg(
            Message {
                content_type: name.to_lowercase(),
                content: Content::Questions(question)
            }
        ))
    }

    Err("Builtin question bad argument".to_owned())
}