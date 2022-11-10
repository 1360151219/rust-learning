use std::collections::HashMap;
use std::fmt::Error;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till1, take_while},
    character::complete::multispace0,
    combinator::{map, map_res, value as nom_value},
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq)]
// json的值
pub enum JsonValue {
    Str(String),
    Boolean(bool),
    Null,
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
// nom api捕获的基本都是到捕获字符之前的字符串～

fn sp(i: &str) -> IResult<&str, &str> {
    let spaces = " \n\r\t";
    take_while(|c| spaces.contains(c))(i)
}

fn num(i: &str) -> IResult<&str, f64> {
    double(i)
}

fn string(i: &str) -> IResult<&str, &str> {
    context(
        "string",
        alt((tag("\"\""), delimited(tag("\""), esc, tag("\"")))),
    )(i)
}

fn esc(i: &str) -> IResult<&str, &str> {
    escaped(normal, '\\', escapable)(i)
}

fn escapable(i: &str) -> IResult<&str, &str> {
    context(
        "escapable",
        alt((tag("t"), tag("n"), tag("r"), tag("\\"), tag("\""))),
    )(i)
}

fn normal(i: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c == '\\' || c == '"' || c.is_ascii_control())(i)
}

fn value(i: &str) -> IResult<&str, JsonValue> {
    context(
        "value",
        delimited(
            multispace0,
            alt((
                null,
                map(boolean, JsonValue::Boolean),
                map(num, JsonValue::Num),
                map(string, |s| JsonValue::Str(String::from(s))),
                map(array, |a| JsonValue::Array(a)),
                map(object, |o| JsonValue::Object(o)),
            )),
            multispace0,
        ),
    )(i)
}

fn boolean(i: &str) -> IResult<&str, bool> {
    let parse_true = nom_value(true, tag("true"));
    let parse_false = nom_value(false, tag("false"));
    alt((parse_true, parse_false))(i)
}

fn null(i: &str) -> IResult<&str, JsonValue> {
    map_res(tag("null"), |_| Ok::<_, Error>(JsonValue::Null))(i)
}

fn array(i: &str) -> IResult<&str, Vec<JsonValue>> {
    context(
        "array",
        delimited(
            tag("["),
            separated_list0(tag(","), delimited(multispace0, value, multispace0)),
            tag("]"),
        ),
    )(i)
}

fn key(i: &str) -> IResult<&str, &str> {
    delimited(multispace0, string, multispace0)(i)
}

fn object(i: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    context(
        "object",
        map(
            delimited(
                tag("{"),
                separated_list0(tag(","), separated_pair(key, tag(":"), value)),
                tag("}"),
            ),
            |tuple_vec| {
                tuple_vec
                    .into_iter()
                    .map(|(k, v)| (String::from(k), v))
                    .collect()
            },
        ),
    )(i)
}

pub fn root(i: &str) -> IResult<&str, JsonValue> {
    delimited(
        multispace0,
        alt((map(object, JsonValue::Object), map(array, JsonValue::Array))),
        multispace0,
    )(i)
}

#[test]
fn test_num() {
    let (_, n) = num("1e9").unwrap();
    assert_eq!(n, 1000000000.0);
}

#[test]
fn test_sp() {
    let (_, s) = sp(" \n\r I am Bob \n\r I am Alice").unwrap();
    assert_eq!(s, " \n\r ");
}

#[test]
fn test_string() {
    let (_, s) = string("\"abc\"").unwrap();
    assert_eq!(s, "abc");
}
#[test]
fn test_escape1() {
    let (esc, res) = esc("a\rbcd;\"").unwrap();
    assert_eq!(esc, "\rbcd;\"");
    assert_eq!(res, "a");
}

#[test]
fn test_escape2() {
    // 捕获到转译字符之前的
    let (esc, res) = esc("abcd;\"efg").unwrap();
    assert_eq!(esc, "\"efg");
    assert_eq!(res, "abcd;");
}

#[test]
fn test_value_bool() {
    let (_, v) = value("true").unwrap();
    assert_eq!(v, JsonValue::Boolean(true));
}

#[test]
fn test_value_null() {
    let (_, v) = value("null").unwrap();
    assert_eq!(v, JsonValue::Null);
}

#[test]
fn test_value_num() {
    let (_, v) = value("123").unwrap();
    assert_eq!(v, JsonValue::Num(123.0));
}

#[test]
fn test_value_str() {
    let (_, v) = value("\"abc\"").unwrap();
    assert_eq!(v, JsonValue::Str(String::from("abc")));
}

#[test]
fn test_value_arr() {
    let (_, v) = value("[1, 2, 3 ]").unwrap();
    assert_eq!(
        v,
        JsonValue::Array(vec![
            JsonValue::Num(1.0),
            JsonValue::Num(2.0),
            JsonValue::Num(3.0)
        ])
    );
}

#[test]
fn test_key() {
    let (_, v) = key(" \"abc\" ").unwrap();
    assert_eq!(v, "abc");
}

#[test]
fn test_obj1() {
    assert_eq!(object(r#"{}"#), Ok(("", HashMap::new())));
    let (_, v) = object("{\"a\":\"1\",\"b\":2}").unwrap();
    let mut m = HashMap::new();
    m.insert(String::from("a"), JsonValue::Str(String::from("1")));
    m.insert(String::from("b"), JsonValue::Num(2.0));
    assert_eq!(v, m);
}

#[test]
fn test_obj2() {
    let mut hash = HashMap::new();
    hash.insert(String::from("key"), JsonValue::Str(String::from("val")));
    hash.insert(
        String::from("arr"),
        JsonValue::Array(vec![
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
            JsonValue::Null,
        ]),
    );
    assert_eq!(
        object(r#"{"key": "val"  , "arr" :    [true, false, null]}"#),
        Ok(("", hash))
    );
}

#[test]
fn test_json() {
    let s = "{ \"a\"\t: 42,\"b\": [ \"x\", \"y\", 12 ],\"c\":{\"d\":100}}";
    dbg!(object(s));
}
