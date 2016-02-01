use std::result::Result;
use std::num::ParseFloatError;
use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter.by_ref())))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = try!(parse_term(&mut iter.by_ref()));
    loop {
        value = match peek_next_symbol(&mut iter.by_ref()) {
            Some('+') => try!(do_addition(value, &mut iter.by_ref())),
            Some('-') => try!(do_substitution(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = try!(parse_arg(&mut iter.by_ref()));
    loop {
        value = match peek_next_symbol(&mut iter.by_ref()) {
            Some('×') => try!(do_multiplication(value, &mut iter.by_ref())),
            Some('÷') => try!(do_division(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = vec![];
    loop {
        match peek_next_symbol(&mut iter.by_ref()) {
            Some(c @ '0'...'9') | Some(c @ '.') => { iter.next(); value.push(c) },
            Some('(') => { skip_symbol(&mut iter.by_ref()); let result = try!(parse_expression(&mut iter.by_ref())); skip_symbol(&mut iter.by_ref()); return Ok(result) },
            Some(_) | None => break,
        }
    }
    value.iter().map(|c| *c).collect::<String>().parse::<f64>()
}

fn peek_next_symbol(iter: &mut Peekable<Chars>) -> Option<char> {
    iter.peek().map(|c| *c)
}

fn skip_symbol(iter: &mut Peekable<Chars>) {
    iter.next();
}

fn do_addition(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value + try!(parse_term(&mut iter.by_ref())))
}

fn do_substitution(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplication(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_arg(&mut iter.by_ref())))
}

fn do_division(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_arg(&mut iter.by_ref())))
}