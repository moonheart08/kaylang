use chumsky::prelude::*;

use crate::{parsing::*, Db};

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token>>, extra::Err<Rich<'src, char, Span>>> 
{
    let bool = 
        just("true").to(Token::Boolean(true))
        .or(just("false").to(Token::Boolean(false)));

    let float = 
        text::int(10)
            .then(just('.').then(text::digits(10)))
            .slice()
            .from_str()
            .unwrapped()
            .map(Token::Float);

    let int = custom::<_, &str, _, extra::Err<Rich<'src, char, Span>>>(|inp| {
        let offs = inp.offset();

        let text = inp.parse(text::int(10));
        
        if let Ok(v) = text {
            let value = u128::from_str_radix(v, 10).map_err(|_| Rich::<'src, char, Span>::custom(inp.span_since(offs), "Couldn't parse integer."))?;

            return Ok(Token::Integer(value));
        }

        return Err(Rich::<'src, char, Span>::custom(inp.span_since(offs), "Couldn't parse integer."));
    });

    let braces = choice(
        [
            just("(").to(Token::OpenParen),
            just(")").to(Token::CloseParen),
            just("{").to(Token::OpenBrace),
            just("}").to(Token::CloseBrace),
            just("[").to(Token::OpenSquareBracket),
            just("]").to(Token::CloseSquareBracket)
        ]
    ).labelled("brace").as_context();

    let symbols = choice(
        [
            just("&&").to(Token::LogicAnd),
            just("||").to(Token::LogicOr),
            just("^^").to(Token::LogicXor),
            just("==").to(Token::Equals),
            just("!=").to(Token::NotEquals),
            just(">=").to(Token::GreaterThanEq),
            just("<=").to(Token::LessThanEq),
            just("=>").to(Token::BigArrow),
            just("->").to(Token::ArrowRight),
            just("<-").to(Token::ArrowLeft),
            just("=").to(Token::Assign),
            just("#").to(Token::Hash),
            just("+").to(Token::Plus),
            just("-").to(Token::Minus),
            just("*").to(Token::Multiply),
            just("/").to(Token::Divide),
            just("%").to(Token::Modulo),
            just("&").to(Token::BitAnd),
            just("|").to(Token::BitOr),
            just("^").to(Token::BitXor),
            just("~").to(Token::BitNot),
            just(",").to(Token::Comma),
            just(".").to(Token::Dot),
            just(";").to(Token::Semicolon),
            just(",").to(Token::Colon),
            just("<").to(Token::LessThan),
            just(">").to(Token::GreaterThan),
            just("!").to(Token::LogicNot),
        ]
    ).labelled("valid token").as_context();

    let words = choice(
        [
            text::keyword::<_, _, _, extra::Err<Rich<'src, char, Span>>>("has").to(Token::Has),
            text::keyword("is").to(Token::Is),
            text::keyword("self").to(Token::TokSelf),
            text::keyword("typeof").to(Token::Typeof),
            text::keyword("sizeof").to(Token::Sizeof),
            text::keyword("public").to(Token::Public),
            text::keyword("scoped").to(Token::Scoped),
            text::keyword("private").to(Token::Private),
            text::keyword("module").to(Token::Module),
            text::keyword("bundle").to(Token::Bundle),
            text::keyword("struct").to(Token::Struct),
            text::keyword("enum").to(Token::Enum),
            text::keyword("union").to(Token::Union),
            text::keyword("trait").to(Token::Trait),
            text::keyword("impl").to(Token::Impl),
            text::keyword("func").to(Token::Func),
            text::keyword("out").to(Token::Out),
            text::keyword("in").to(Token::In),
            text::keyword("ref").to(Token::Ref),
            text::keyword("where").to(Token::Where),
            text::keyword("return").to(Token::Return),
            text::keyword("if").to(Token::If),
            text::keyword("match").to(Token::Match),
            text::keyword("break").to(Token::Break),
            text::keyword("continue").to(Token::Continue),
            text::keyword("while").to(Token::While),
            text::keyword("do").to(Token::While),
            text::keyword("for").to(Token::For),
            text::keyword("field").to(Token::Field),
            text::keyword("let").to(Token::Let),
            text::keyword("as").to(Token::As),
        ]
    ).or(bool).labelled("keyword").as_context();

    let str_ = 
        none_of(['\\', '"'])
        .or(
            just('\\').ignore_then(none_of(['x', 'u'])).map(|x| match x {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '0' => '\0',
                v => v,
            })
        )
        .or(
            just('\\')
            .ignore_then(just('x').ignored())
            .then(text::digits(16).exactly(2).slice())
            .map(|x: ((), &'src str)| {
                char::from_u32(u8::from_str_radix(x.1, 16).unwrap() as u32).unwrap()
            })
        )
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(Token::Str)
        .labelled("string literal").as_context();

    let comment_line = 
        just("//")
            .then(any().and_is(just('\n').not()).repeated())
            .padded();

    let comment = comment_line;

    let values = float.or(int).or(str_);

    let ident = words.or(text::ident().map(|x: &str| x.to_string()).map(Token::Ident));

    let token = values.or(braces).or(symbols).or(ident).labelled("valid token").as_context();

    return token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect();
}

type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token, Span, &'tokens [(Token, Span)]>;

pub type Spanned<T> = (T, Span);

pub fn postlexer<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Vec<Spanned<Token>>,
    extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone 
{
    return any().map_with_span(|x: Token, s| (x, s)).repeated().collect();
}

#[salsa::tracked]
pub fn lex(db: &dyn Db, input: File) -> LexedFile 
{
    let contents = input.contents(db).clone();
    let lexer = lexer();
    let parsed = lexer.parse(&contents);
    LexedFile::new(db, input.path(db), parsed.to_owned().into_output().unwrap(), parsed.errors().map(|x| x.clone().into_owned()).collect())
}