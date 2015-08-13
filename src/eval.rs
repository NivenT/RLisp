use parser::*;
use errors::*;
use native::*;
use types::*;
use env::*;

use errors::LispError::*;
use types::Function::*;
use types::Special::*;
use types::Native::*;
use types::Datum::*;
use types::List::*;
use types::Atom::*;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub fn eval(form: &Datum, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		LIST(ref l)	=> eval_list(l, env),
		ATOM(ref a) => eval_atom(a, env),
		ref e @ _	=> Err(INVALID_FORM(e.clone()))
	}
}

fn eval_list(form: &List, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		CONS(ref car, ref cdr) 	=> {
			let f = eval(car, env);
			if f.is_err() {
				return f;
			} match f.ok().unwrap() {
				FUNCTION(func) 	=> {
					match *cdr.clone() {
						LIST(args) 	=> apply(func, args.get_items(), env),
						_			=> Err(INVALID_ARG_LIST(*cdr.clone()))
					}
				},
				ref e @ _			=> Err(UNKNOWN_FUNCTION(e.clone()))
			} 
		},
		NIL						=> Ok(LIST(NIL))
	}
}

fn eval_atom(form: &Atom, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		SYMBOL(ref a) 	=> env.get(a),
		ref e @ _		=> Ok(ATOM(e.clone()))
	}
}

fn apply(func: Function, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	match func {
		SPECIAL(ref s)	=> apply_special(s, args, env),
		NATIVE(ref n)	=> apply_native(n, args, env),
		LAMBDA(ref l)	=> apply_lambda(l, args, env)
	}
}

fn apply_native(func: &Native, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	let mut items: Vec<Datum> = Vec::with_capacity(args.len());
	for i in 0..args.len() {
		let res = eval(&args[i], env);
		if res.is_err() {
			return res;
		} else {
			items.push(res.ok().unwrap());
		}
	}

	match *func {
		ADD			=> add(items),
		SUB			=> sub(items),
		MUL			=> mul(items),
		DIV			=> div(items),
		LIST_FUNC 	=> list(items),
		CONS_FUNC	=> cons(items),
		CAR 		=> car(items),
		CDR			=> cdr(items),
		NTH_CDR		=> nth_cdr(items),
		NTH			=> nth(items),
		GT 			=> greater_than(items),
		GE 			=> greater_equal(items),
		LT 			=> less_than(items),
		LE 			=> less_equal(items),
		MATH_EQ     => math_equal(items),
		MOD			=> lisp_mod(items),
		LOAD 		=> load(items, env),
		//_			=> Err(NOT_YET_IMPLEMENTED(FUNCTION(NATIVE(*func))))
	}
}

fn apply_special(func: &Special, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	match *func {
		DEFINE		=> define(args, env),
		IF 			=> lisp_if(args, env),
		LAMBDA_FUNC => lambda(args, env),
		DEFUN 		=> defun(args, env),
		QUOTE 		=> quote(args),
		BACKQUOTE   => backquote(args, env),
		LET 		=> let_lisp(args, env),
		LET_STAR 	=> let_star(args, env),
		PROGN 		=> progn(args, env),
		//_			=> Err(NOT_YET_IMPLEMENTED(FUNCTION(SPECIAL(*func))))
	}
}

fn apply_lambda(func: &Lambda, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != func.args.len() {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), func.args.len()));
	}

	let mut params: Vec<Datum> = Vec::with_capacity(args.len());
	for i in 0..args.len() {
		let res = eval(&args[i], env);
		if res.is_err() {
			return res;
		} else {
			params.push(res.ok().unwrap());
		}
	}

	env.push_map(&func.env);
	for param_arg in params.into_iter().zip(&func.args) {
		env.set(param_arg.1.clone(), param_arg.0);
	}
	let res = eval(&func.body, env);
	env.pop();

	return res;
}

fn define(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	match args.len() {
		2 => {
			if let ATOM(SYMBOL(sym)) = args[0].clone() {
				let res = eval(&args[1], env);
				if res.is_err() {
					return res;
				} else {
					match env.get(&sym) {
						Ok(ref e @ FUNCTION(SPECIAL(_))) |
						Ok(ref e @ FUNCTION(NATIVE(_))) => return Err(OVERRIDE_RESERVED(e.clone())),
						_								=> return Ok(env.set(sym, res.ok().unwrap()))
					}
				}
			}
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "symbol"))
		},
		e @ _ => Err(INVALID_NUMBER_OF_ARGS(e, 2))
	}
}

fn is_true(cond: Datum) -> bool {
	match cond {
		LIST(NIL)	=> false,
		_			=> true
	}
}

fn lisp_if(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	match args.len() {
		2 | 3 	=> {
			let res = eval(&args[0], env);
			if res.is_err() {
				res
			} else if is_true(res.ok().unwrap()) {
				eval(&args[1], env)
			} else if args.len() == 3 {
				eval(&args[2], env)
			} else {
				Ok(LIST(NIL))
			}
		},
		e @ _	=> Err(INVALID_NUMBER_OF_ARGS(e, 3))
	}
}

fn lambda(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 2 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 2));
	} 

	if let LIST(params) = args[0].clone() {
		let mut arguments: Vec<String> = vec![];
		let params = params.get_items();

		for param in params {
			if let ATOM(SYMBOL(name)) = param {
				arguments.push(name);
			} else {
				return Err(INVALID_ARGUMENT_TYPE(param, "symbol"));
			}
		}

		let func = Lambda{args: arguments, body: Box::new(args[1].clone()), env: env.top()};
		return Ok(FUNCTION(LAMBDA(func)));
	} else {
		return Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "list"));
	}
}

fn defun(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 3 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 3));
	} 

	let lam = lambda(vec!(args[1].clone(), args[2].clone()), env);
	if lam.is_err() {
		return lam;
	} else if let ATOM(SYMBOL(name)) = args[0].clone() {
		match env.get(&name) {
			Ok(ref e @ FUNCTION(SPECIAL(_))) |
			Ok(ref e @ FUNCTION(NATIVE(_))) => return Err(OVERRIDE_RESERVED(e.clone())),
			_								=> return Ok(env.set(name, lam.ok().unwrap()))
		}
	} else {
		return Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "symbol"))
	}
}

fn quote(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	}  else {
		Ok(args[0].clone())
	}
}

fn backquote(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else {
		backquote_helper(&args[0], env)
	}
}

fn backquote_helper(arg: &Datum, env: &mut Env) -> Result<Datum, LispError> {
	match *arg {
		ref e @ ATOM(_) | ref e @ FUNCTION(_) 	=> Ok(e.clone()),
		LIST(ref lst)							=> {
			if lst.car() == ATOM(SYMBOL("COMMA".to_string())) {
				eval(&lst.get_items()[1], env) //assumes list is of form (COMMA item)
			} else {
				let mut items = lst.get_items();
				for i in 0..items.len() {
					let res = backquote_helper(&items[i], env);
					if res.is_err() {
						return res;
					} else {
						items[i] = res.ok().unwrap();
					}
				}

				Ok(LIST(List::from_vec(items)))
			}
		}
	}
}

fn let_lisp(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 2 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 2));
	}

	if let LIST(ref lst) = args[0] {
		let mut map: HashMap<String, Datum> = HashMap::new();
		for item in lst.get_items() {
			if let LIST(lst) = item {
				let binding = lst.get_items();
				if binding.len() != 2 {
					return Err(INVALID_ARGUMENT_TYPE(LIST(lst), "list of length 2"))
				}

				if let ATOM(SYMBOL(ref name)) = binding[0] {
					let res = eval(&binding[1], env);
					if res.is_err() {
						return res;
					} else {
						map.insert(name.clone(), res.ok().unwrap());
					}
				} else {
					return Err(INVALID_ARGUMENT_TYPE(binding[0].clone(), "symbol"))
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(item, "list"))
			}
		}

		env.push_map(&map);
		let res = eval(&args[1], env);
		env.pop();
		res
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "list"))
	}
}

fn let_star(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 2 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 2));
	}

	if let LIST(ref lst) = args[0] {
		env.push();
		for item in lst.get_items() {
			if let LIST(lst) = item {
				let binding = lst.get_items();
				if binding.len() != 2 {
					return Err(INVALID_ARGUMENT_TYPE(LIST(lst), "list of length 2"))
				}

				if let ATOM(SYMBOL(ref name)) = binding[0] {
					let res = eval(&binding[1], env);
					if res.is_err() {
						return res;
					} else {
						env.set(name.clone(), res.ok().unwrap());
					}
				} else {
					return Err(INVALID_ARGUMENT_TYPE(binding[0].clone(), "symbol"))
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(item, "list"))
			}
		}

		let res = eval(&args[1], env);
		env.pop();
		res
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "list"))
	}
}

fn progn(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	let mut res = Ok(LIST(NIL));
	for item in args {
		res = eval(&item, env);
		if res.is_err() {
			return res;
		}
	}
	res
}

pub fn load(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(STRING(ref file_path)) = args[0] {
		let mut file = match File::open(file_path.clone()) {
			Ok(f) 		=> f,
			Err(why) 	=> return Err(CANNOT_OPEN_FILE(format!("{}", why)))
		};
		let mut contents = String::new();

		file.read_to_string(&mut contents).ok().expect("Failed read file to string");
		eval(&parse(&mut tokenize(&format!("(progn {})", contents))), env)
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "string"))
	}
}