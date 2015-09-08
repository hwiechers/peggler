//TODO: Figure out error reporting
#[derive(Debug, Eq, PartialEq)]
pub struct ParseError;

pub type ParseResult<'a, T> = Result<(T, &'a str), ParseError>;

pub type Parser<T> = Fn(&str) -> ParseResult<T>;

pub fn optional<'a, T>(result: ParseResult<'a, T>, input: &'a str) -> ParseResult<'a, Option<T>> {
    match result {
        Ok((value, remaining)) => Ok((Some(value), remaining)),
        Err(_) => Ok((None, input))
    }
}

pub fn many<'a, T>(items: Vec<T>, input: &'a str) -> ParseResult<'a, Vec<T>> {
    Ok((items, input))
}

#[macro_export]
macro_rules! rule(
    ($name:ident:$result_type:ty = $($rest:tt)+) => {
        fn $name(input: &str) -> $crate::ParseResult<$result_type> {
            expression!(input, __, $($rest)+)
        }
    }
);

#[macro_export]
macro_rules! expression(
    //action
    ($input:ident, $label:ident, => $action:block) => {{
        let result = $action;
        Ok((result, $input))
    }};

    //and parser
    ($input:ident, $label:ident, &$parser:tt) => {{
        match expression!($input, $label, $parser) {
            Ok(_) => Ok(((), $input)),
            Err(err) => Err(err)
        }
    }};

    //not parser
    ($input:ident, $label:ident, !$parser:tt) => {{
        match expression!($input, $label, $parser) {
            Ok(_) => Err(ParseError),
            Err(_) => Ok(((), $input)),
        }
    }};

    //not parser into sequence
    //-> subexpression into sequence
    ($input:ident, $label:ident, !$parser:tt $($rest:tt)+) => {{
        expression!($input, $label, (!$parser) $($rest)+)
    }};

    //and parser into sequence
    //-> subexpression into sequence
    ($input:ident, $label:ident, &$parser:tt $($rest:tt)+) => {{
        expression!($input, $label, (&$parser) $($rest)+)
    }};

    //subexpression into action into choice
    //-> subexpression into choice
    ($input:ident, $label:ident, ($($parsers:tt)+) => $action:block / $($rest:tt)+) => {{
        expression!($input, $label, ($($parsers)+ => $action) / $($rest)+)
    }};

    //subexpression into action into sequence
    //-> subexpression into choice
    ($input:ident, $label:ident, ($($parsers:tt)+) => $action:block $($rest:tt)+) => {{
        expression!($input, $label, ($($parsers)+ => $action) $($rest)+)
    }};

    //string ...
    //-> subexpression ...
    ($input:ident, $label:ident, [$string:expr] $($rest:tt)+) => {{
        expression!($input, $label, ([$string]) $($rest)+)
    }};

    //string
    ($input:ident, $label:ident, [$string:expr]) => {{
        if $input.starts_with($string) {
            Ok(($string, &$input[$string.len()..]))
        } else {
            Err($crate::ParseError)
        }
    }};

    //parser into and parser ...
    //-> subexpression into subexpression ...
    ($input:ident, $label:ident, ($($parsers:tt)+) & $next:tt $($rest:tt)*) => {{
        expression!($input, $label, ($($parsers)+) (& $next) $($rest)*)
    }};

    //subexpression into not parser ...
    //-> subexpression into subexpression ...
    ($input:ident, $label:ident, ($($parsers:tt)+) !$next:tt $($rest:tt)*) => {{
        expression!($input, $label, ($($parsers)+) (! $next) $($rest)*)
    }};

    //subexpression
    ($input:ident, $label:ident, ($($parsers:tt)+)) => {{
        expression!($input, $label, $($parsers)+)
    }};

    //subexpression?
    ($input:ident, $label:ident, ($($parsers:tt)+)?) => {{
        $crate::optional(expression!($input, $label, $($parsers)+), $input)
    }};

    //subexpression+
    ($input:ident, $label:ident, ($($parsers:tt)+)+) => {{
        match expression!($input, $label, $($parsers)+) {
            Ok((first_item, mut input)) => {
                let mut result = Vec::new();
                result.push(first_item);
                loop {
                    let parse_result = expression!(input, $label, $($parsers)+);
                    match parse_result {
                        Ok(values) => {
                            result.push(values.0);
                            input = values.1;
                        },
                        Err(_) => {
                            break
                        }
                    }
                }

                $crate::many(result, input)
            },
            Err(err) => Err(err),
        }
    }};

    //subexpression*
    //-> mapped from ((subexpression)+)?
    ($input:ident, $label:ident, ($($parsers:tt)+)*) => {{
        let result = expression!($input, $label, (($($parsers)+)+)?);
        match result {
            Ok((Some(value), remaining)) => Ok((value, remaining)),
            Ok((None, _)) => Ok((vec![], $input)),
            _ => unreachable!(),
        }
    }};

    //subexpression? ...
    //-> subexpression ...
    ($input:ident, $label:ident, ($($parsers:tt)+)? $($rest:tt)+) => {{
        expression!($input, $label, (($($parsers)+)?) $($rest)+)
    }};

    //subexpression+ ...
    //-> subexpression ...
    ($input:ident, $label:ident, ($($parsers:tt)+)+ $($rest:tt)+) => {{
        expression!($input, $label, (($($parsers)+)+) $($rest)+)
    }};

    //subexpression* ...
    //-> subexpression ...
    ($input:ident, $label:ident, ($($parsers:tt)+)* $($rest:tt)+) => {{
        expression!($input, $label, (($($parsers)+)*) $($rest)+)
    }};

    //subexpression into choice
    ($input:ident, $label:ident, ($($parsers:tt)+) / $($rest:tt)+) => {{
        match expression!($input, $label, $($parsers)+) {
            result @ Ok((_, _)) => result,
            Err(_) => expression!($input, __, $($rest)+),
        }
    }};

    //subexpression into sequence
    ($input:ident, $label:ident, ($($parsers:tt)+) $($rest:tt)+) => {{
        match expression!($input, __, $($parsers)+) {
            Ok(($label, input)) => expression!(input, __, $($rest)+),
            Err(err) => Err(err),
        }
    }};

    //label
    ($input:ident, $old_label:ident, $label:ident:$($rest:tt)+) => {{
        expression!($input, $label, $($rest)+)
    }};

    //parser into choice
    //-> subexpression into choice
    ($input:ident, $label:ident, $parser:tt / $($rest:tt)+) => {{
        expression!($input, $label, ($parser) / $($rest)+)
    }};

    //parser? ...
    //-> subexpression? ...
    ($input:ident, $label:ident, $parser:tt? $($rest:tt)*) => {{
        expression!($input, $label, ($parser)? $($rest)*)
    }};

    //parser+ ...
    //-> subexpression+ ...
    ($input:ident, $label:ident, $parser:tt+ $($rest:tt)*) => {{
        expression!($input, $label, ($parser)+ $($rest)*)
    }};

    //parser* ...
    //-> subexpression* ...
    ($input:ident, $label:ident, $parser:tt* $($rest:tt)*) => {{
        expression!($input, $label, ($parser)* $($rest)*)
    }};

    //parser into sequence
    //-> subexpression into sequence
    ($input:ident, $label:ident, $parser:tt $($rest:tt)+) => {{
        expression!($input, $label, ($parser) $($rest)+)
    }};

    //parser identifier
    ($input:ident, $label:ident, $parser:ident) => {
        $parser($input)
    };
);

#[cfg(test)]
mod tests {
    pub use super::{ParseError, ParseResult};

    fn a(input: &str) -> ParseResult<u32> {
        if input.starts_with("a") {
            Ok((1, &input[1..]))
        } else {
            Err(ParseError)
        }
    }

    fn b(input: &str) -> ParseResult<u32> {
        if input.starts_with("b") {
            Ok((2, &input[1..]))
        } else {
            Err(ParseError)
        }
    }

    fn c(input: &str) -> ParseResult<u32> {
        if input.starts_with("c") {
            Ok((3, &input[1..]))
        } else {
            Err(ParseError)
        }
    }

    mod rule_definition;
    mod sequences;
    mod choices;
    mod subexpressions;
    mod string_expressions;
    mod optional;
    mod one_or_more;
    mod zero_or_more;
    mod and;
    mod not;
}
