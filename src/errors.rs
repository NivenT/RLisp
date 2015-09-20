use types::*;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LispError {
	INVALID_ARGUMENT_TYPE(Datum, &'static str),
	INVALID_NUMBER_OF_ARGS(usize, usize),
	UNBOUND_VARIABLE(String),
	UNKNOWN_FUNCTION(Datum),
	INVALID_ARG_LIST(Datum),
	OVERRIDE_RESERVED(Datum),
	CANNOT_OPEN_FILE(String),
	_NOT_YET_IMPLEMENTED(Datum),
	MULTIPLE_REST_ARGS,
	DIVISION_BY_ZERO,
	MISMATCHED_BRACKETS,
	NO_INPUT
}

use self::LispError::*;

impl LispError {
	pub fn message(&self) -> String {
		match self.clone() {
			INVALID_ARGUMENT_TYPE(act, exp) => 
				format!("Invalid argument: {} should be of type {}", act, exp),
			INVALID_NUMBER_OF_ARGS(act, exp) =>
				format!("Invalid number of arguments: {} provided but {} expected", act, exp),
			UNBOUND_VARIABLE(name) =>
				format!("Unbound variable: No value set for {}", name),
			UNKNOWN_FUNCTION(x) =>
				format!("Unkown function: {} is not a known function or lambda expression", x),
			INVALID_ARG_LIST(lst) =>
				format!("Invalid arguments: {} should be a list", lst),
			CANNOT_OPEN_FILE(reason) =>
				format!("Cannot open file: {}", reason),
			OVERRIDE_RESERVED(x) =>
				format!("Attempted to override reserved symbol: {}", x),
			_NOT_YET_IMPLEMENTED(x) =>
				format!("{} has not been implemented yet", x),
			MULTIPLE_REST_ARGS =>
				format!("Error: arg list should contain at most 1 &rest argument"),
			DIVISION_BY_ZERO =>
				format!("Attempted to divide by zero"),
			MISMATCHED_BRACKETS =>
				format!("Attempted to close a parenthesis with a square bracket or vice versa"),
			NO_INPUT =>
				format!("No value")
		}
	}
}