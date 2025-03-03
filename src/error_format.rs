pub mod data;

use crate::data::ast::Interval;
pub use data::{CustomError, ErrorInfo};

// pub fn get_error_message(error_code: ErrorKind, code_error: &[u8]) -> String {
//     match error_code {
//         ErrorKind::Custom(val) if val == ParserErrorType::StepDuplicateError as u32 => {
//             "Step name already exists".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::AssignError as u32 => {
//             "Missing as after remember statement".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::AsError as u32 => {
//             "in as module (var as var_name)".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::NoAscii as u32 => {
//             "non-ascii idents are not supported".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::GotoStepError as u32 => {
//             "Missing label name after goto".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::AcceptError as u32 => {
//             "Flow argument expect Accept identifier".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::LeftBraceError as u32 => {
//             "Missing start of block { ".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::RightBraceError as u32 => {
//             "Arguments inside brace bad format or brace missing".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::LeftParenthesesError as u32 => {
//             "list elemt type ( ... ) not found".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::RightParenthesesError as u32 => {
//             "Arguments inside parentheses bad format or ) missing".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::RightBracketError as u32 => {
//             "Arguments inside parentheses bad format or ] missing".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::DoubleQuoteError as u32 => {
//             "\" maybe missing".to_string()
//         }
//         ErrorKind::Custom(val) if val == ParserErrorType::DoubleBraceError as u32 => {
//             "}} maybe missing".to_string()
//         }
//         ErrorKind::Eof => {
//             let mut s: String = str::from_utf8(code_error)
//                 .expect("error in from_utf8")
//                 .to_owned();
//             if let Some(val) = s.find('\n') {
//                 s.truncate(val)
//             };
//             s.to_string()
//         }
//         e => e.description().to_owned(),
//     }
// }

pub fn format_error<I>(
    interval: Interval,
    error_code: CustomError<I>,
    _code_error: &[u8],
) -> ErrorInfo {
    // let message = get_error_message(error_code, code_error);
    // ErrorInfo { interval, message }
    let message = error_code.error;
    ErrorInfo { interval, message }
}
