use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::{map, map_res},
    sequence::tuple,
    AsChar, IResult,
};
//delimited 这个组合子函数, 这个组合子函数接受三个类似 tag("xx") 这样的基本函数, 依次应用这三个函数, 如果成功, 则返回第二个函数解析的结果.
#[derive(Debug, PartialEq, Eq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    // 将x进制转换为u8的形式
    // dbg!(input);
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_hex_digit() // 0..=9,a..=f,A..=F
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    //nom 提供了扩展性更好的 take_while_m_n, m, n 分为 最少和最多匹配数
    //map_res 类似map
    map_res(take_while_m_n(2, 2, |c| is_hex_digit(c)), |s| from_hex(s))(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    // tuple 接受一组组合子, 将组合子按顺序应用到输入上, 然后按顺序返回以元组返回解析结果
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
    Ok((input, Color { red, green, blue }))
}

#[test]
fn test_tag() {
    fn parser(s: &str) -> IResult<&str, &str> {
        tag("#")(s)
    }
    let (input, t) = parser("#ffffff").unwrap();
    assert_eq!(input, "ffffff");
    assert_eq!(t, "#");
}
#[test]
fn test_parse() {
    let (_, s) = hex_color("#ffeeaa").unwrap();
    assert_eq!(
        s,
        Color {
            red: 255,
            green: 238,
            blue: 170
        }
    )
}
