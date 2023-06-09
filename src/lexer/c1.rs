use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // Keywords
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,

    // Operatoren
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token(">")]
    Grt,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    // Other Tokens
    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    // Termvariables
    #[regex(r#"[0-9]+"#)]
    ConstInt,

    #[regex(r#"((([0-9]+)?\.[0-9]+)([eE][-+]?[0-9]+)?)|([0-9]+([eE][-+]?[0-9]+))"#)]
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    #[regex(r#""[^\n"]*""#)]
    ConstString,

    #[regex("[a-zA-Z]+([0-9]|[a-zA-Z])*")]
    Id,

    // Comments
    #[regex(r#"/\*([^\*]|\*[^/])*\*/"#, logos::skip)]
    CComment,

    #[regex(r#"//[^\n]*\n"#, logos::skip)]
    CppComment,

    #[regex(r"\s", logos::skip)]
    Whitespace,

    #[error]
    Error,
}
