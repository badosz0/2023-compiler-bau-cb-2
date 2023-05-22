use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex(r"//.*\n", logos::skip)]
    LineComment,

    #[regex(r"/\*([^*])*\*/", logos::skip)]
    BlockComment,

    #[token("bool")]
    KwBoolean,

    #[token("return")]
    KwReturn,

    #[token("if")]
    KwIf,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("void")]
    KwVoid,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("do")]
    KwDo,

    #[token("while")]
    KwWhile,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("==")]
    Eq,

    #[token("=")]
    Assign,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token("||")]
    Or,

    #[token("&&")]
    And,

    #[token("*")]
    Asterisk,

    #[token("+")]
    Plus,

    #[token("!=")]
    Neq,

    #[regex(r"(true|false)+", priority = 2)]
    ConstBoolean,
  
    #[regex(r"([0-9]+[.][0-9]+|[.][0-9]+|[0-9]+[e][0-9]+)")]
    ConstFloat,

    #[regex(r"[0-9]+")]
    ConstInt,

    #[regex(r#""([^"\\]|\\.)*""#)]
    ConstString,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Id,
}
