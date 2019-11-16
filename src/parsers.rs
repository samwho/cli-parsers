use nom::character::complete::{alpha1, space1, space0};

use nom::error::ParseError;
use nom::{AsChar, InputTakeAtPosition};

fn not_space0<T, E: ParseError<T>>(input: T) -> nom::IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input.split_at_position_complete(|item| {
    let c = item.clone().as_char();
    (c == ' ' || c == '\t')
  })
}

named!(uppercase_name<&str, &str>, take_while1!(|c: char| c.is_uppercase()));
named!(env_var<&str, (&str, &str)>, separated_pair!(uppercase_name, tag!("="), alpha1));
named!(env_vars<&str, Vec<(&str, &str)>>, separated_list!(space1, env_var));
named!(args<&str, Vec<&str>>, separated_list!(space1, not_space0));
named!(pub bash_cmd<&str, (Option<Vec<(&str, &str)>>, &str, Option<Vec<&str>>)>,
  do_parse!(
    env: opt!(env_vars) >>
    space0 >>
    cmd: alpha1 >>
    space0 >>
    args: opt!(args) >>
    (env, cmd, args)
  )
);