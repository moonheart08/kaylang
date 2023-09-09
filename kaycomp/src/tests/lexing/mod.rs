use crate::parsing::{self, Token};
use chumsky::prelude::*;

pub fn lex(inp: &str) -> impl Iterator<Item = Token> 
{
    parsing::lexer().parse(inp).into_output().unwrap().into_iter().map(|x| x.0)
}

pub fn assert_tok(iter: &mut impl Iterator<Item = Token>, t: Token) 
{
    assert_eq!(iter.next().unwrap(), t);
}

#[test]
pub fn lex_integer() 
{
    let txt = r##"1 -1"##;
    let mut lexed = lex(txt);
    // We don't process signs at lex-time.
    assert_tok(&mut lexed, Token::Integer(1));
    assert_tok(&mut lexed, Token::Minus);
    assert_tok(&mut lexed, Token::Integer(1));
}

#[test]
pub fn lex_float() 
{
    let txt = r##"1.0 -1.0"##;
    let mut lexed = lex(txt);
    // We don't process signs at lex-time.
    assert_tok(&mut lexed, Token::Float(1.0));
    assert_tok(&mut lexed, Token::Minus);
    assert_tok(&mut lexed, Token::Float(1.0));
}


#[test]
pub fn lex_str() 
{
    let mut hello_world = lex(r##""hello, world!""##);
    assert_tok(&mut hello_world, Token::Str("hello, world!".to_string()));

    let mut escapes = lex(r#""\"hi! \n\n\"""#);
    assert_tok(&mut escapes, Token::Str("\"hi! \n\n\"".to_string()));

    let mut hex_escapes = lex(r#""\x48\x69"#);
    assert_tok(&mut hex_escapes, Token::Str("Hi".to_string()))
}