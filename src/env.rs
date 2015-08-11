use errors::*;
use types::*;

use errors::LispError::*;
use types::Function::*;
use types::Special::*;
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

		map.insert(">".to_string(), FUNCTION(NATIVE(GT)));
		map.insert(">=".to_string(), FUNCTION(NATIVE(GE)));
		map.insert("<".to_string(), FUNCTION(NATIVE(LT)));
		map.insert("<=".to_string(), FUNCTION(NATIVE(LE)));
		map.insert("=".to_string(), FUNCTION(NATIVE(MATH_EQ)));		

		map.insert("LIST".to_string(), FUNCTION(NATIVE(LIST_FUNC)));
		map.insert("CONS".to_string(), FUNCTION(NATIVE(CONS_FUNC)));
		map.insert("CAR".to_string(), FUNCTION(NATIVE(CAR)));
		map.insert("CDR".to_string(), FUNCTION(NATIVE(CDR)));

		map.insert("NTHCDR".to_string(), FUNCTION(NATIVE(NTH_CDR)));
		map.insert("NTH".to_string(), FUNCTION(NATIVE(NTH)));

		map.insert("IF".to_string(), FUNCTION(SPECIAL(IF)));
		map.insert("LET".to_string(), FUNCTION(SPECIAL(LET)));
		map.insert("LET*".to_string(), FUNCTION(SPECIAL(LET_STAR)));
		map.insert("PROGN".to_string(), FUNCTION(SPECIAL(PROGN)));
		map.insert("QUOTE".to_string(), FUNCTION(SPECIAL(QUOTE)));
		map.insert("BACKQUOTE".to_string(), FUNCTION(SPECIAL(BACKQUOTE)));
		map.insert("DEFINE".to_string(), FUNCTION(SPECIAL(DEFINE)));
		map.insert("DEFUN".to_string(), FUNCTION(SPECIAL(DEFUN)));
		map.insert("LAMBDA".to_string(), FUNCTION(SPECIAL(LAMBDA_FUNC)));

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

	pub fn set(&mut self, key: String, val: Datum) -> Datum {
		self.env_stack.last_mut().unwrap().insert(key, val.clone());
		val
	}
}