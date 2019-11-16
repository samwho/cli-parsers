#[allow(dead_code)]

#[macro_use] extern crate nom;

mod parsers;

use std::result;

#[derive(Debug)]
pub enum Error<'a> {
    Nom(nom::Err<(&'a str, nom::error::ErrorKind)>),
}

impl<'a> std::convert::From<nom::Err<(&'a str, nom::error::ErrorKind)>> for Error<'a> {
 fn from(err: nom::Err<(&'a str, nom::error::ErrorKind)>) -> Error<'a> {
     Error::Nom(err)
 }
}

pub type Result<'a, T> = result::Result<T, Error<'a>>;

#[derive(Debug)]
pub struct Cmd<'a> {
 env: Vec<(&'a str, &'a str)>,
 cmd: &'a str,
 args: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    pub fn env(&self) -> &Vec<(&'a str, &'a str)> {
        &self.env
    }

    pub fn cmd(&self) -> &'a str {
        self.cmd
    }

    pub fn args(&self) -> &Vec<&'a str> {
        &self.args
    }
}

pub fn parse_bash(input: &str) -> Result<Cmd> {
    let (_, (env, cmd, args)) = parsers::bash_cmd(input)?;
    Ok(Cmd{
        env: env.unwrap_or_default(),
        cmd: cmd,
        args: args.unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bash_full() {
        let cmd = parse_bash("HELLO=world FOO=bar ls -ltrh   foo/bar").expect("parsing failed");
        assert_eq!(cmd.env(), &vec!(("HELLO", "world"), ("FOO", "bar")));
        assert_eq!(cmd.cmd(), "ls");
        assert_eq!(cmd.args(), &vec!("-ltrh", "foo/bar"));
    }

    #[test]
    fn parse_bash_cmd() {
        let cmd = parse_bash("ls").expect("parsing failed");
        assert_eq!(cmd.env().is_empty(), true);
        assert_eq!(cmd.cmd(), "ls");
        assert_eq!(cmd.args().is_empty(), true);
    }

    #[test]
    fn parse_bash_cmd_with_env() {
        let cmd = parse_bash("HELLO=world ls").expect("parsing failed");
        assert_eq!(cmd.env(), &vec!(("HELLO", "world")));
        assert_eq!(cmd.cmd(), "ls");
        assert_eq!(cmd.args().is_empty(), true);
    }

    #[test]
    fn parse_bash_cmd_with_args() {
        let cmd = parse_bash("ls -ltrh test").expect("parsing failed");
        assert_eq!(cmd.env().is_empty(), true);
        assert_eq!(cmd.cmd(), "ls");
        assert_eq!(cmd.args(), &vec!("-ltrh", "test"));
    }

    #[test]
    fn parse_bash_err_no_cmd() {
        parse_bash("HELLO=world").expect_err("expected error");
    }
}
