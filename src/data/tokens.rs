use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

pub const PORT: &str = "3002";

pub const WHITE_SPACE: &str = " \t\n\r";
pub const INLINE_COMMENT: &str = "//";

pub const START_COMMENT: &str = "/*";
pub const END_COMMENT: &str = "*/";

pub const ADDITION: &str = "+";
pub const SUBTRACTION: &str = "-";
pub const DIVIDE: &str = "/";
pub const MULTIPLY: &str = "*";
pub const REMAINDER: &str = "%";
pub const NOT: &str = "!";

pub const EQUAL: &str = "==";
pub const NOT_EQUAL: &str = "!=";
pub const ASSIGN: &str = "=";

pub const OR: &str = "||";
pub const AND: &str = "&&";

pub const GREATER_THAN_EQUAL: &str = ">=";
pub const LESS_THAN_EQUAL: &str = "<=";
pub const GREATER_THAN: &str = ">";
pub const LESS_THAN: &str = "<";

pub const COMMA: &str = ",";
pub const DOT: &str = ".";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";
pub const DOUBLE_QUOTE: &str = "\"";
pub const UNDERSCORE: char = '_';

pub const L_PAREN: &str = "(";
pub const R_PAREN: &str = ")";
pub const L_BRACE: &str = "{";
pub const R_BRACE: &str = "}";
pub const L_BRACKET: &str = "[";
pub const R_BRACKET: &str = "]";
pub const L2_BRACE: &str = "{{";
pub const R2_BRACE: &str = "}}";

pub const FOREACH: &str = "foreach";
pub const IF: &str = "if";
pub const ELSE: &str = "else";

pub const IMPORT: &str = "import";
pub const AS: &str = "as";
pub const IN: &str = "in";
pub const DO: &str = "do";
pub const FROM: &str = "from";
pub const EVENT: &str = "event";

pub const FLOW: &str = "flow";
pub const FILE: &str = "file";
pub const STEP: &str = "step";
pub const SAY: &str = "say";
pub const USE: &str = "use";
pub const HOLD: &str = "hold";
pub const GOTO: &str = "goto";
pub const MATCH: &str = "match";
pub const DEFAULT: &str = "default";
pub const REMEMBER: &str = "remember";
pub const _METADATA: &str = "_metadata";
pub const BREAK: &str = "break";

pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const NULL: &str = "null";

pub static RESERVED: &[&str] = &[
    FOREACH, IF, ELSE, IMPORT, AS, IN, DO, FROM, EVENT, FLOW, FILE, STEP, SAY, USE, HOLD, GOTO,
    MATCH, DEFAULT, REMEMBER, _METADATA, TRUE, FALSE, NULL, BREAK,
];

// TODO: at some point (when function doesnt have a key like find(in)) ut back in inside reserved
pub static UTILISATION_RESERVED: &[&str] = &[
    FOREACH, IF, ELSE, IMPORT, AS, DO, FROM, FLOW, FILE, STEP, SAY, USE, HOLD, GOTO, MATCH,
    DEFAULT, REMEMBER, BREAK,
];

pub static ASSIGNATION_RESERVED: &[&str] = &[
    FOREACH, IF, ELSE, IMPORT, AS, DO, FROM, EVENT, FLOW, FILE, STEP, SAY, USE, HOLD, GOTO, MATCH,
    DEFAULT, REMEMBER, _METADATA, TRUE, FALSE, NULL, BREAK,
];

pub const TYPING: &str = "Typing";
pub const WAIT: &str = "Wait";
pub const TEXT: &str = "Text";
pub const URL: &str = "Url";
pub const IMAGE: &str = "Image";
pub const ONE_OF: &str = "OneOf";
pub const SHUFFLE: &str = "Shuffle";
pub const LENGTH: &str = "Length";
pub const FIND: &str = "Find";
pub const RANDOM: &str = "Random";
pub const FLOOR: &str = "Floor";
pub const VIDEO: &str = "Video";
pub const AUDIO: &str = "Audio";

pub const QUESTION: &str = "Question";
pub const BUTTON: &str = "Button";
pub const OBJECT: &str = "Object";
pub const FN: &str = "Fn";

pub static BUILT_IN: &[&str] = &[
    TYPING, WAIT, TEXT, URL, IMAGE, ONE_OF, SHUFFLE, LENGTH, FIND, RANDOM, FLOOR, VIDEO, AUDIO,
    QUESTION, BUTTON, OBJECT, FN,
];

pub const MEMORY: &str = "memory";

pub const FROM_FILE: &str = "FromFile";
pub const GET_VALUE: &str = "GetValue";
pub const FIRST: &str = "first";
