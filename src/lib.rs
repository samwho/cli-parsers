#[macro_use]
extern crate nom;

#[macro_use]
extern crate maplit;

mod parsers;

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Nom(String),
}

impl<'a> std::convert::From<nom::Err<(&'a str, nom::error::ErrorKind)>> for Error {
    fn from(err: nom::Err<(&'a str, nom::error::ErrorKind)>) -> Error {
        Error::Nom(format!("{:?}", err))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Nom(s) => &s,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Nom(_) => None, // TODO: fix this
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Nom(s) => fmt::Display::fmt(&s, f),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Cmd {
    env: HashMap<String, String>,
    cmd: String,
    args: Vec<String>,
}

impl Cmd {
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn cmd(&self) -> &str {
        &self.cmd
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

pub fn parse_bash(input: &str) -> Result<Cmd> {
    let (_, (env, cmd, args)) = parsers::bash_cmd(input)?;
    Ok(Cmd {
        env: env
            .unwrap_or_default()
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect(),
        cmd: cmd.to_string(),
        args: args
            .unwrap_or_default()
            .iter()
            .map(|s| (*s).to_string())
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bash_full() {
        let cmd = parse_bash("HELLO=world FOO=bar ls -ltrh   foo/bar").expect("parsing failed");
        assert_eq!(
            cmd.env(),
            &hashmap!("HELLO".to_string() => "world".to_string(), "FOO".to_string() => "bar".to_string())
        );
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
        assert_eq!(
            cmd.env(),
            &hashmap!("HELLO".to_string() => "world".to_string())
        );
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
