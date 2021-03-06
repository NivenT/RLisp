use errors::*;
use types::*;

use errors::LispError::*;
use types::Function::*;
use types::Special::*;
use types::Native::*;
use types::Datum::*;

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
		map.insert("MOD".to_string(), FUNCTION(NATIVE(MOD)));
		map.insert("POWI".to_string(), FUNCTION(NATIVE(POWI)));
		map.insert("POWR".to_string(), FUNCTION(NATIVE(POWR)));

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
		map.insert("MOST".to_string(), FUNCTION(NATIVE(MOST)));

		map.insert("LOAD".to_string(), FUNCTION(NATIVE(LOAD)));

		map.insert("FLOOR".to_string(), FUNCTION(NATIVE(FLOOR)));
		map.insert("CEIL".to_string(), FUNCTION(NATIVE(CEIL)));

		map.insert("TYPE".to_string(), FUNCTION(NATIVE(TYPE)));

		map.insert("ATOM?".to_string(), FUNCTION(NATIVE(IS_ATOM)));
		map.insert("LIST?".to_string(), FUNCTION(NATIVE(IS_LIST)));
		map.insert("CONS?".to_string(), FUNCTION(NATIVE(IS_CONS)));
		map.insert("SYMBOL?".to_string(), FUNCTION(NATIVE(IS_SYMBOL)));

		map.insert("EQUAL?".to_string(), FUNCTION(NATIVE(EQUAL)));

		map.insert("WRITE-TO-STRING".to_string(), FUNCTION(NATIVE(WRITE_TO_STRING)));
		map.insert("READ-FROM-STRING".to_string(), FUNCTION(NATIVE(READ_FROM_STRING)));
		map.insert("STRING-CONCAT".to_string(), FUNCTION(NATIVE(STRING_CONCAT)));
		map.insert("PRINT".to_string(), FUNCTION(NATIVE(PRINT)));
		map.insert("FORMAT".to_string(), FUNCTION(NATIVE(FORMAT)));

		map.insert("NOT".to_string(), FUNCTION(NATIVE(NOT)));
		map.insert("SET".to_string(), FUNCTION(NATIVE(SET)));
		map.insert("GENSYM".to_string(), FUNCTION(NATIVE(GENSYM)));
		map.insert("EVAL".to_string(), FUNCTION(NATIVE(EVAL)));
		map.insert("APPLY".to_string(), FUNCTION(NATIVE(APPLY)));

		map.insert("RAND-INT".to_string(), FUNCTION(NATIVE(RANDINT)));
		map.insert("RAND-BOOL".to_string(), FUNCTION(NATIVE(RANDBOOL)));
		map.insert("RAND-REAL".to_string(), FUNCTION(NATIVE(RANDREAL)));	

		map.insert("IF".to_string(), FUNCTION(SPECIAL(IF)));
		map.insert("LET".to_string(), FUNCTION(SPECIAL(LET)));
		map.insert("LET*".to_string(), FUNCTION(SPECIAL(LET_STAR)));
		map.insert("PROGN".to_string(), FUNCTION(SPECIAL(PROGN)));
		map.insert("QUOTE".to_string(), FUNCTION(SPECIAL(QUOTE)));
		map.insert("BACKQUOTE".to_string(), FUNCTION(SPECIAL(BACKQUOTE)));
		map.insert("DEFINE".to_string(), FUNCTION(SPECIAL(DEFINE)));
		map.insert("DEFUN".to_string(), FUNCTION(SPECIAL(DEFUN)));
		map.insert("LAMBDA".to_string(), FUNCTION(SPECIAL(LAMBDA_FUNC)));
		map.insert("TIME".to_string(), FUNCTION(SPECIAL(TIME)));
		map.insert("MACRO".to_string(), FUNCTION(SPECIAL(MACRO_FUNC)));
		map.insert("DEFMACRO".to_string(), FUNCTION(SPECIAL(DEFMACRO)));
		map.insert("MACROEXPAND".to_string(), FUNCTION(SPECIAL(MACROEXPAND)));

		Env{env_stack: vec![map, HashMap::new()]}
	}

	pub fn get(&self, key: &String) -> Result<Datum, LispError> {
		let len = self.env_stack.len();
		for i in 0..len {
			if self.env_stack[len-i-1].contains_key(key) {
				return Ok(self.env_stack[len-i-1][key].clone())
			}
		}
		Err(UNBOUND_VARIABLE(key.clone()))
	}

	pub fn set(&mut self, key: String, val: Datum) -> Datum {
		self.env_stack.last_mut().unwrap().insert(key, val.clone());
		val
	}

	pub fn set_bot(&mut self, key: String, val: Datum) -> Datum {
		self.env_stack[2].insert(key, val.clone());
		val
	}

	pub fn push(&mut self) {
		self.env_stack.push(HashMap::new());
	}

	pub fn push_map(&mut self, map: &HashMap<String, Datum>) {
		self.env_stack.push(map.clone());
	}

	pub fn pop(&mut self) {
		self.env_stack.pop();
	}

	pub fn top(&mut self) -> HashMap<String, Datum> {
		self.env_stack.last().unwrap().clone()
	}

	pub fn join(&self) -> Vec<(String, Datum)> {
		self.env_stack.clone().into_iter()
			.flat_map(|map| map.into_iter())
			.collect()
	}
}