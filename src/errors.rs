use types::*;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LispError {
	INVALID_ARGUMENT_TYPE(Datum, &'static str),
	NOT_ENOUGH_ARGUMENTS(usize, usize),
	TOO_MANY_ARGUMENTS(usize, usize),
	UNBOUND_VARIABLE(String),
	INVALID_FORM(Datum),
	UNKNOWN_FUNCTION(Datum),
	INVALID_ARG_LIST(Datum),
	NOT_YET_IMPLEMENTED(Datum),
	OVERRIDE_RESERVED(Datum),
	MISMATCHED_BRACKETS,
	NO_INPUT
}

use self::LispError::*;

impl LispError {
	pub fn message(&self) -> String {
		match self.clone() {
			INVALID_ARGUMENT_TYPE(act, exp) => 
				format!("Invalid argument: {} should be of type {}", act, exp),
			NOT_ENOUGH_ARGUMENTS(act, exp) =>
				format!("Too few arguments: {} provided but {} expected", act, exp),
			TOO_MANY_ARGUMENTS(act, exp) =>
				format!("Too many arguments: {} provided but {} expected", act, exp),
			UNBOUND_VARIABLE(name) =>
				format!("Unbound variable: No value set for {}", name),
			INVALID_FORM(x) =>
				format!("Invalid form: {} can not be evaluated", x),
			UNKNOWN_FUNCTION(x) =>
				format!("Unkown function: {} is not a known function or lambda expression", x),
			INVALID_ARG_LIST(lst) =>
				format!("Invalid arguments: {} should be a list", lst),
			NOT_YET_IMPLEMENTED(x) =>
				format!("{} has not been implemented yet", x),
			OVERRIDE_RESERVED(x) =>
				format!("Attempted to override reserved symbol: {}", x),
			MISMATCHED_BRACKETS =>
				format!("Attempted to close a parenthesis with a square bracket or vice versa"),
			NO_INPUT =>
				format!("No value")
		}
	}
}