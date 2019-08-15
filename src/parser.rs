use crate::App;
use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_while};
use nom::character::complete::{alphanumeric1, char, one_of};
use nom::combinator::{cut, map};
use nom::error::{convert_error, ErrorKind, ParseError, VerboseError};
use nom::multi::separated_list;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::error;

fn sp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
}

fn parse_str<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    //    let chars = "*?.";
    //    let allowed = take_while(move |c| {
    //        let x = c as u8;
    //        chars.contains(c)
    //    });
    //escaped(allowed, '\\', one_of("\"n\\"))(input)
    escaped(alphanumeric1, '\\', one_of("\"n\\"))(input)
}

fn string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(char('\"'), cut(terminated(parse_str, char('\"'))))(input)
}

fn parse_exec<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(tag("exec"), cut(preceded(sp, string)))(input)
}

fn parse_copy<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(tag("copy"), cut(preceded(sp, string)))(input)
}

fn parse_copy_glob<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(tag("copy_glob"), cut(preceded(sp, string)))(input)
}

fn parse_action<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, crate::Action, E> {
    preceded(
        sp,
        alt((
            map(parse_exec, |s| crate::Action::Execute(s.into())),
            map(parse_copy, |s| crate::Action::Copy(s.into())),
            map(parse_copy_glob, |s| crate::Action::CopyGlob(s.into())),
        )),
    )(input)
}

fn parse_actions<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<crate::Action>, E> {
    preceded(
        char('{'),
        cut(terminated(
            separated_list(preceded(sp, char(';')), parse_action),
            preceded(sp, char('}')),
        )),
    )(input)
}

fn root<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let (input, _) = sp(input)?;
    let (input, _) = tag("app")(input)?;
    let (input, _) = sp(input)?;
    string(input)
}

pub fn parse<'a>(input: &'a str) -> Result<App, Box<dyn error::Error>> {
    match root::<VerboseError<&str>>(input) {
        Ok((_, name)) => Ok(App {
            name: name.into(),
            actions: vec![],
        }),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(convert_error(input, e).into()),
        Err(nom::Err::Incomplete(e)) => Err(format!("{:?}", e).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_parse_actions() {
        assert_eq!(
            parse_actions::<(&str, ErrorKind)>(
                r#"{
                
            exec  "ls";   
            copy "file"
            
            }"#
            ),
            Ok((
                "",
                vec![
                    crate::app::Action::Execute("ls".into()),
                    crate::app::Action::Copy("file".into())
                ]
            ))
        );
    }
    #[test]
    fn test_parse_exec() {
        assert_eq!(
            parse_action::<(&str, ErrorKind)>(r#"exec  "ls""#),
            Ok(("", crate::app::Action::Execute("ls".into())))
        );
    }
    #[test]
    fn test_parse_copy() {
        assert_eq!(
            parse_action::<(&str, ErrorKind)>(r#"copy  "xml""#),
            Ok(("", crate::app::Action::Copy("xml".into())))
        );
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(string::<(&str, ErrorKind)>(r#""hello""#), Ok(("", "hello")));
    }

    #[test]
    fn test_parse_name() {
        let app = parse(r#" app "name" "#);
        match app {
            Ok(app) => assert_eq!(
                app,
                App {
                    name: "name".into(),
                    actions: vec![]
                }
            ),
            Err(e) => panic!("failed {}", e),
        }
    }
}
