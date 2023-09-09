#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    None,
    /*
     * Type tokens
     */

    Boolean(bool),
    Integer(u128),
    Float(f64),
    Str(String),

    /*
     * Operators
     */

    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    BitAnd,         // &
    LogicAnd,       // &&
    BitOr,          // |
    LogicOr,        // ||
    BitXor,         // ^
    LogicXor,       // ^^
    BitNot,         // ~
    LogicNot,       // !
    Assign,         // =
    Equals,         // ==
    NotEquals,      // !=
    GreaterThan,    // >
    GreaterThanEq,  // >=
    LessThan,       // <
    LessThanEq,     // <=
    ArrowRight,     // ->
    ArrowLeft,      // <-
    BigArrow,       // =>
    Hash,           // #

    /*
     * Word Tokens
     */

    // General
    Is,             // is
    TokSelf,        // self

    // sorta-functions
    Typeof,         // typeof
    Sizeof,         // sizeof

    // Visibility
    Public,         // public
    Scoped,         // scoped
    Private,           // private
    Module,         // module
    Bundle,         // bundle
    
    // Types
    Struct,         // struct
    Enum,           // enum
    Union,          // union
    Trait,          // trait
    Impl,           // impl

    // Functions
    Func,           // func
    Out,            // out
    In,             // in
    Ref,            // ref

    // Control flow
    Return,         // return
    If,             // if
    Match,          // match
    Break,          // break
    Continue,       // continue
    While,          // while
    Do,             // do
    For,            // for
    Let,            // let
    As,             // as

    // Constraints
    Where,          // where
    Has,            // has
    Field,          // field

    /*
     * Symbols
     */
    // ( )
    OpenParen,
    CloseParen,
    // { }
    OpenBrace,
    CloseBrace,
    // [ ]
    OpenSquareBracket,
    CloseSquareBracket,
    
    Comma,
    Dot,
    Semicolon,
    Colon,

    /*
     * General
     */

    Ident(String),
    Unknown(char)
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::None => write!(f, "ERRTOK"),
            Token::Unknown(v) => write!(f, "UNK:{}", v),
            Token::Boolean(v) => write!(f, "{}", v),
            Token::Integer (value) => write!(f, "{}i", value),
            Token::Float(v) => write!(f, "{}f", v),
            Token::Str(v) => write!(f, "{}", v),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::BitAnd => write!(f, "&"),
            Token::LogicAnd => write!(f, "&&"),
            Token::BitOr => write!(f, "|"),
            Token::LogicOr => write!(f, "||"),
            Token::BitXor => write!(f, "^"),
            Token::LogicXor => write!(f, "^^"),
            Token::BitNot => write!(f, "~"),
            Token::LogicNot => write!(f, "!"),
            Token::Assign => write!(f, "="),
            Token::Equals => write!(f, "=="),
            Token::NotEquals => write!(f, "!="),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanEq => write!(f, ">="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanEq => write!(f, "<="),
            Token::ArrowRight => write!(f, "->"),
            Token::ArrowLeft => write!(f, "<-"),
            Token::BigArrow => write!(f, "=>"),
            Token::Hash => write!(f, "#"),
            Token::Has => write!(f, "has"),
            Token::Is => write!(f, "is"),
            Token::TokSelf => write!(f, "self"),
            Token::Typeof => write!(f, "typeof"),
            Token::Sizeof => write!(f, "sizeof"),
            Token::Public => write!(f, "public"),
            Token::Scoped => write!(f, "scoped"),
            Token::Private => write!(f, "private"),
            Token::Module => write!(f, "module"),
            Token::Bundle => write!(f, "bundle"),
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::Union => write!(f, "union"),
            Token::Trait => write!(f, "trait"),
            Token::Impl => write!(f, "impl"),
            Token::Func => write!(f, "func"),
            Token::Out => write!(f, "out"),
            Token::In => write!(f, "in"),
            Token::Ref => write!(f, "ref"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Match => write!(f, "match"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::While => write!(f, "while"),
            Token::Let => write!(f, "let"),
            Token::Do => write!(f, "do"),
            Token::For => write!(f, "for"),
            Token::As => write!(f, "as"),
            Token::Where => write!(f, "where"),
            Token::Field => write!(f, "field"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::OpenSquareBracket => write!(f, "["),
            Token::CloseSquareBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ","),
            Token::Ident(v) => write!(f, "i#{}", v),
        }
    }
}