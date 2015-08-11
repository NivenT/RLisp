use errors::*;
use funcs::*;
use types::*;

use errors::LispError::*;
use types::Function::*;
use types::Native::*;
use types::Datum::*;
use types::List::*;

use std::collections::HashMap;

pub struct Env {
	env_stack: Vec<HashMap<String,Datum>>
}

impl Env {
	pub fn new() -> Env {
		let mut map: HashMap<String,Datum> = HashMap::new();
		map.insert("+".to_string(), FUNCTION(NATIVE(ADD)));
		map.insert("-".to_string(), FUNCTION(NATIVE(SUB)));
		map.insert("*".to_string(), FUNCTION(NATIVE(MUL)));
		map.insert("/".to_string(), FUNCTION(NATIVE(DIV)));

		map.insert("LIST".to_string(), FUNCTION(NATIVE(LIST_FUNC)));
		map.insert("CONS".to_string(), FUNCTION(NATIVE(CONS_FUNC)));
		map.insert("CAR".to_string(), FUNCTION(NATIVE(CAR)));
		map.insert("CDR".to_string(), FUNCTION(NATIVE(CDR)));

		map.insert("NTHCDR".to_string(), FUNCTION(NATIVE(NTH_CDR)));
		map.insert("NTH".to_string(), FUNCTION(NATIVE(NTH)));

		map.insert("NIL".to_string(), LIST(NIL));

		Env{env_stack: vec![map]}
	}

	pub fn get(&self, key: &String) -> Result<Datum, LispError> {
		let len = self.env_stack.len();
		for i in 0..len {
			if self.env_stack[len-i-1].contains_key(key) {
				return Ok(self.env_stack[len-i-1][key].clone())
			}
		}
		Err(UNBOUND_VARIABLE)
	}

	pub fn apply(&self, func: Function, args: List) -> Result<Datum, LispError> {
		match func {
			NATIVE(ADD)			=> add(args),
			NATIVE(SUB)			=> sub(args),
			NATIVE(MUL)			=> mul(args),
			NATIVE(DIV)			=> div(args),
			NATIVE(LIST_FUNC) 	=> list(args),
			NATIVE(CONS_FUNC)	=> cons(args),
			NATIVE(CAR) 		=> car(args),
			NATIVE(CDR)			=> cdr(args),
			NATIVE(NTH_CDR)		=> nth_cdr(args),
			NATIVE(NTH)			=> nth(args),
			_					=> Err(UNKNOWN_FUNCTION)
		}
	}
}