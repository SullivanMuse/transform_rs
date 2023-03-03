use std::{collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1},
    combinator::{map, recognize, verify},
    multi::{many0, separated_list1},
    sequence::{pair, delimited, preceded},
    IResult,
};

struct Env {
    parent: Option<Rc<Env>>,
    map: HashMap<String, Expr>,
}

#[derive(Clone, Debug)]
struct Sym(Rc<String>);

#[derive(Clone, Debug)]
enum ExprBox {
    /* data */
    // 123_456
    Int(i64),

    // :xyz
    Name(Sym),

    // x y z
    List(Vec<Expr>),

    /* expressions */
    // :xyz | :(x y z)
    Quote(Expr),

    // fn name args body
    Fn(Sym, Vec<Sym>, Expr),

    // lam args body
    Lam(Vec<Sym>, Expr),

    // let k v b
    Let(Sym, Expr, Expr),

    // set k v
    Set(Sym, Expr),

    // do x y z
    Do(Vec<Expr>),
}

#[derive(Clone, Debug)]
struct Expr(Rc<ExprBox>);

fn int(s: &str) -> IResult<&str, Expr> {
    map(
        recognize(pair(digit1, many0(pair(tag("_"), digit1)))),
        |s: &str| Expr(Rc::new(ExprBox::Int(s.parse::<i64>().unwrap()))),
    )(s)
}

fn name(s: &str) -> IResult<&str, Expr> {
    const kw: &[&'static str] = &[
        "fn",
        "macro",
        "lam",
        "let",
        "set",
        "do",
    ];
    map(
        verify(recognize(pair(alpha1, many0(alt((tag("_"), alphanumeric1))))), |s| !kw.contains(s)),
        |s: &str| Expr(Rc::new(ExprBox::Name(Sym(Rc::new(s.to_string()))))),
    )(s)
}

fn paren(s: &str) -> IResult<&str, Expr> {
    delimited(pair(tag("("), multispace0), expr, pair(multispace0, tag(")")))(s)
}

fn quote(s: &str) -> IResult<&str, Expr> {
    map(preceded(pair(tag(":"), multispace0), atom), |e| Expr(Rc::new(ExprBox::Quote(e))))(s)
}

fn atom(s: &str) -> IResult<&str, Expr> {
    alt((quote, int, name, paren))(s)
}

fn list(s: &str) -> IResult<&str, Expr> {
    map(separated_list1(multispace1, atom), |xs: Vec<Expr>| {
        Expr(Rc::new(ExprBox::List(xs)))
    })(s)
}

fn expr(s: &str) -> IResult<&str, Expr> {
    alt((list, atom))(s)
}

fn repl<F>(f: F)
where
    F: Fn(&str),
{
    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();
        if s == "exit" {
            return;
        }
        f(s);
    }
}

fn main() {
    repl(|s| {
        dbg!(expr(s));
    });
}
