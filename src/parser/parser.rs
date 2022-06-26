use crate::tokenizer::{TokenType, Token, Keyword, Char, Brace, BracketType, Operator};

pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Value(i64),
    SymbolValue(String),
}

pub enum Instruction {
    Named(u64, String),                     // optcode, name
    Number(u64),                            // optcode
}

pub enum Statement {
    Label(String),                          // name of the label
    Instruction(Instruction, Expression),   // instruction, argument
    MacroCall(String, Vec<Expression>),     // name of macro, arguments for macro call
}

pub enum AstNode {
    Program(Vec<AstNode>),                  // children of program

    Include(String),                        // include path
    Define(String, Expression),             // define name, define value
    MacroCreate(String, Vec<Statement>),    // name of macro, macro body

    Statement(Statement),
    Expression(Expression),

    UntilCollection(Vec<AstNode>)
}

pub enum ParserError {
    Default(String),
    UnbalancedBracketsError,
}

pub struct ParserResult {
    length: usize,
    node: AstNode,
}

pub trait Parser {
    fn digest(&self, tokens: &[TokenType]) -> Result<ParserResult, ParserError>;

    fn digest_all(&self, tokens: &[Token]) -> Result<AstNode, ParserError> {
        todo!()
    }
}

// pub struct IncludeParser {}
// impl Parser for IncludeParser {
//     fn digest(&self, tokens: &[TokenType]) -> Result<AstNode, ParserError> {
//         match tokens {
//             [
//                 TokenType::Char(Char::Percent),
//                 TokenType::Keyword(Keyword::Include),
//                 TokenType::StringLiteral(include_path),
//                 ..
//             ] => {
//                 Ok(AstNode::Include(include_path.to_owned()))
//             }
//             _ => Err(ParserError::Default)
//         }
//     }
// }

macro_rules! parser {
    ($name:ident, $res_node:expr, $($pattern:tt)*) => {
        pub struct $name {}
        impl Parser for $name {
            fn digest(&self, tokens: &[TokenType]) -> Result<ParserResult, ParserError> {
                match tokens {
                    [$($pattern)*] => Ok($res_node),
                    _ => Err(ParserError::Default("pattern diddnt match".to_owned())),
                }
            }
        }
    };
}

parser! {
    IncludeParser,
    ParserResult {
        node: AstNode::Include(include_path.to_owned()),
        length: 3
    },
    
    TokenType::Char(Char::Percent),
    TokenType::Keyword(Keyword::Include),
    TokenType::StringLiteral(include_path),
    ..
}

parser! {
    DefineParser,
    ParserResult {
        node: AstNode::Define(define_name.to_owned(), Expression::Value(*define_value)),
        length: 4,
    },

    TokenType::Char(Char::Percent),
    TokenType::Keyword(Keyword::Define),
    TokenType::Symbol(define_name),
    TokenType::NumberLiteral(define_value),
    ..
}

parser! {
    MacroCreateParser,
    {
        let res = ParserUntil::new(|tok| match tok {
            TokenType::Keyword(Keyword::End) => true,
            _ => false,
        }, StatementParser).digest(rest)?;

        if let AstNode::UntilCollection(nodes) = res.node {
            let nodes = nodes.iter().map(|node| if let AstNode::Statement(st) = node { st } else )
            ParserResult {
                node: AstNode::MacroCreate(macro_name, ),
                length: res.length + 3 + 1,
            }
        } else {  }

    },

    TokenType::Keyword(Keyword::Macro),
    TokenType::Symbol(macro_name),
    TokenType::Char(Char::Bang),
    rest @ ..
}

pub struct ExpressionParser {}
impl Parser for ExpressionParser {
    fn digest(&self, tokens: &[TokenType]) -> Result<AstNode, ParserError> {
        match tokens {
            [TokenType::NumberLiteral(v), ..] => { Ok(AstNode::Expression(Expression::Value(*v))) }
            [TokenType::Symbol(v), ..] => { Ok(AstNode::Expression(Expression::SymbolValue(v.to_owned()))) }
            _ => Err(ParserError::Default("expressions not supported".to_owned()))
        }
        // match tokens {
        //     // parentheses
        //     [TokenType::Brace(Brace::Round(BracketType::Opening)), ..] => {
        //         let mut depth = 1;
        //         let inner: Vec<TokenType> = tokens.iter()
        //             .skip(1)
        //             .take_while(|v| match v {
        //                 TokenType::Brace(Brace::Round(BracketType::Closing)) => {
        //                     depth -= 1;
        //                     depth == 0
        //                 },
        //                 TokenType::Brace(Brace::Round(BracketType::Opening)) => {
        //                     depth += 1;
        //                     false
        //                 }
        //                 _ => true,
        //             })
        //             .map(|v| v.to_owned())
        //             .collect();

        //         // unbalanced brackets
        //         if depth > 0 { return Err(ParserError::UnbalancedBracketsError); }

        //         match self.digest(&inner)? {
        //             AstNode::Expression(expr) => Ok(AstNode::Expression(expr)),
        //             _ => Err(ParserError::Default("invalid AstNode".to_owned()))
        //         }
        //     }

        //     // addition
        //     [
        //         TokenType::NumberLiteral(_) | TokenType::Symbol(_),
        //         TokenType::Operator(Operator::Add),
        //         TokenType::NumberLiteral(_) | TokenType::Symbol(_),
        //     ] => {

        //     }
        // }
    }
}

pub struct ParserUntil<T: Parser> {
    parser: T,
    stop_cb: fn(TokenType) -> bool,
}
impl<T: Parser> ParserUntil<T> {
    fn new(stop_cb: fn(TokenType) -> bool, parser: T) -> Self {
        Self {
            stop_cb,
            parser,
        }
    }
}
impl<T: Parser> Parser for ParserUntil<T> {


    fn digest(&self, tokens: &[TokenType]) -> Result<ParserResult, ParserError> {
        let mut nodes: Vec<AstNode> = Vec::new();
        let mut length = 0;
        let mut current = &tokens[..];
        while current.len() > 0 && !(self.stop_cb)(current[0]) {
            let parser_result = self.parser.digest(&current)?;
            length += parser_result.length;
            current = &tokens[length..];
        }
        
        Ok(ParserResult {
            length,
            node: AstNode::UntilCollection(nodes)
        })
    }
}

// program
//      include
//          Percent, Keyword(Include), StringLiteral([path])
//      define
//          Percent, Keyword(Define), Symbol([name]), ExpressionParser,
//      macro-create
//          Keyword(Macro), Symbol([name]), Char(Bang), Until<Keyword(End), StatementParser>, Keyword(End)
//
//      [StatementParser]
//          label
//              Symbol([name]), Char(Colon)
//          instruction
//              Keyword(Instruction([value])) | Number([value]), ExpressionParser | expression
//          macro-call
//              Symbol([macro-name]), Char(Bang), While<Expression([argument]), Char(Comma)>

//  ExpressionParser
//      [ExpressionParser], Operation(_), [ExpressionParser]
//  |   Parentheses(Open), [ExpressionParser], Parentheses(Closed)
//  |   NumberLiteral(_)
//  |   Symbol(_)


// steps:
//      collect macros / defines
//      expand macros
//      expand defines
//      collect labels
//      make addresses from labels
//      replace symbols for labels with addresses
//      evaluate expressions
//
//      generate code 


// regexp-parser
//      [...tokens,]                token collection
//      (name:...tokens)            capture group
//      +, *                        quanitfiers
