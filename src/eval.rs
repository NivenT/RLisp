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
use time::PreciseTime;

pub fn eval(form: &Datum, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		LIST(ref l)	=> eval_list(l, env),
		ATOM(ref a) => eval_atom(a, env),
		ref e @ _	=> Ok(e.clone())
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
		LAMBDA(ref l)	=> apply_lambda(l, args, env),
		MACRO(ref m)	=> apply_macro(m, args, env)
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
		ADD					=> add(items),
		SUB					=> sub(items),
		MUL					=> mul(items),
		DIV					=> div(items),
		LIST_FUNC 			=> list(items),
		CONS_FUNC			=> cons(items),
		CAR 				=> car(items),
		CDR					=> cdr(items),
		NTH_CDR				=> nth_cdr(items),
		NTH					=> nth(items),
		GT 					=> greater_than(items),
		GE 					=> greater_equal(items),
		LT 					=> less_than(items),
		LE 					=> less_equal(items),
		MATH_EQ     		=> math_equal(items),
		MOD					=> lisp_mod(items),
		LOAD 				=> load(items, env),
		POWI				=> powi(items),
		POWR				=> powr(items),
		FLOOR  				=> floor(items),
		CEIL 				=> ceil(items),
		TYPE 				=> type_lisp(items),
		IS_ATOM				=> is_atom(items),
		IS_LIST				=> is_list(items),
		IS_CONS				=> is_cons(items),
		IS_SYMBOL			=> is_symbol(items),
		EQUAL 				=> equal(items),
		WRITE_TO_STRING 	=> write_to_string(items),
		READ_FROM_STRING 	=> read_from_string(items),
		STRING_CONCAT		=> string_concat(items),
		NOT 				=> not(items),
		PRINT 				=> print(items),
		SET 				=> set(items, env),
		GENSYM				=> gensym(items, env),
		APPLY 				=> apply_lisp(items, env),
		EVAL 				=> eval_lisp(items, env),
		MOST 				=> most(items),
		RANDINT				=> rand_int(items),
		RANDBOOL			=> rand_bool(items),
		RANDREAL			=> rand_real(items),
		FORMAT 				=> format(items),
		//_					=> Err(_NOT_YET_IMPLEMENTED(FUNCTION(NATIVE(*func))))
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
		TIME 		=> time(args, env),
		MACRO_FUNC  => macro_lisp(args, env),
		DEFMACRO    => defmacro(args, env),
		MACROEXPAND => macroexpand(args, env),
		//_			=> Err(_NOT_YET_IMPLEMENTED(FUNCTION(SPECIAL(*func))))
	}
}

fn apply_lambda(func: &Lambda, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() < func.args.len() {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), func.args.len()));
	}

	let mut params: Vec<Datum> = Vec::with_capacity(func.args.len());
	let mut optional_params: Vec<Datum> = Vec::with_capacity(func.optn.len());
	let mut key_params: Vec<(String, Datum)> = Vec::with_capacity(func.key.len());
	let mut rest_params: Vec<Datum> = Vec::new();

	for i in 0..func.args.len() {
		let res = eval(&args[i], env);
		if res.is_err() {
			return res;
		} else {
			params.push(res.ok().unwrap());
		}
	}

	let mut is_key = false;
	let mut key_name = "";
	for i in func.args.len()..args.len() {
		if let ATOM(SYMBOL(ref name)) = args[i] {
			if name.starts_with(':') && i != args.len()-1 {
				let name = &name[1..];
				if func.contains_key(name.to_string()) && !is_key {
					if !key_params.clone().into_iter().any(|(key, _): (String,_)| key==":".to_string()+name) {
						key_name = &name;
						is_key = true;
						continue;
					}
				}
			} 
		}

		let res = eval(&args[i], env);
		if res.is_err() {
			return res;
		} else if is_key {
			key_params.push((key_name.to_string(), res.ok().unwrap()));
			is_key = false;
		} else if optional_params.len() != func.optn.len() {
			optional_params.push(res.ok().unwrap());
		} else if func.rest != None {
			rest_params.push(res.ok().unwrap());
		} else {
			return Err(INVALID_NUMBER_OF_ARGS(args.len(), func.args.len()));
		}
	}

	env.push_map(&func.env);
	for (param, arg) in params.into_iter().zip(&func.args) {
		env.set(arg.clone(), param);
	}
	for (name, default) in func.optn.clone() {
		let res = eval(&default, env);
		if res.is_err() {
			return res;
		}
		env.set(name.clone(), res.ok().unwrap());
	}
	for (param, arg) in optional_params.into_iter().zip(&func.optn) {
		env.set(arg.0.clone(), param);
	}
	for (name, default) in func.key.clone() {
		let res = eval(&default, env);
		if res.is_err() {
			return res;
		}
		env.set(name.clone(), res.ok().unwrap());
	}
	for (name, val) in key_params {
		env.set(name, val);
	}
	if let Some(name) = func.rest.clone() {
		env.set(name.clone(), LIST(List::from_vec(rest_params)));
	}
	let res = eval(&func.body, env);
	env.pop();
	res
}

fn apply_macro(func: &Lambda, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	let res = macroexpand_helper(func, args, env);
	if res.is_err() {
		res
	} else {
		eval(&res.ok().unwrap(), env)
	}
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
						Ok(FUNCTION(SPECIAL(_))) | Ok(FUNCTION(NATIVE(_))) 
							=> return Err(OVERRIDE_RESERVED(sym)),
						_ 	=> return Ok(env.set_bot(sym, res.ok().unwrap()))
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

fn lambda_contains(sym: String, tree: Datum) -> bool {
	if ATOM(SYMBOL(sym.clone())) == tree {
		true
	} else if let LIST(CONS(ref a, ref b)) = tree {
		if lambda_contains(sym.clone(), *a.clone()) {
			true
		} else {
			lambda_contains(sym, *b.clone())
		}
	} else {
		false
	}
}

fn lambda(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 2 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 2));
	} 

	if let LIST(params) = args[0].clone() {
		let mut arguments: Vec<String> = vec![];
		let mut optn_args: Vec<(String, Datum)> = vec![];
		let mut key_args: Vec<(String, Datum)> = vec![];
		let mut rest_arg: Option<String> = None;
		let params = params.get_items();
		let mut mode: usize = 0; //(0 - normal, 1 - optional,
								 // 2 - rest, 3 - key)

		for param in params {
			if let ATOM(SYMBOL(name)) = param {
				if name == "&OPTIONAL".to_string() {
					mode = 1
				} else if name == "&REST".to_string() {
					mode = 2
				} else if name == "&KEY".to_string() {
					mode = 3
				} else if mode == 1 {
					optn_args.push((name, LIST(NIL)))
				} else if mode == 2 {
					if rest_arg == None {
						rest_arg = Some(name);
					} else {
						return Err(MULTIPLE_REST_ARGS);
					}
				} else if mode == 3 {
					key_args.push((name, LIST(NIL)))
				} else {
					arguments.push(name)
				}
			} else if let LIST(lst) = param.clone() {
				let items = lst.get_items();
				if items.len() != 2 {
					return Err(INVALID_ARGUMENT_TYPE(param, "list of length 2"));
				} else if let ATOM(SYMBOL(name)) = items[0].clone() {
					if mode == 1 {
						optn_args.push((name, items[1].clone()))
					} else if mode == 3 {
						key_args.push((name, items[1].clone()))
					} else {
						return Err(MISPLACED_DEFAULT_VALUE);
					}
				} else {
					return Err(INVALID_ARGUMENT_TYPE(items[1].clone(), "symbol"));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(param, "symbol"));
			}
		}

		return Ok(FUNCTION(LAMBDA(
					Lambda{args: arguments,
						   optn: optn_args,
						   key:  key_args,
						   rest: rest_arg,
						   body: Box::new(args[1].clone()),
						   env: env.top().into_iter()
						   		   .filter(|&(ref key, _)| 
						   		   		lambda_contains(key.clone(),
						   		   						args[1].clone()))
						   		   .collect()}
					)));
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
			Ok(FUNCTION(SPECIAL(_))) | Ok(FUNCTION(NATIVE(_))) 
				=> return Err(OVERRIDE_RESERVED(name)),
			_	=> return Ok(env.set(name, lam.ok().unwrap()))
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
		LIST(NIL)								=> Ok(LIST(NIL)),
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

pub fn time(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else {
		let start = PreciseTime::now();
		let res = eval(&args[0], env);
		println!("Duration of computation: {}", start.to(PreciseTime::now()));
		res
	}
}

pub fn set(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 2 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 2))
	} else if let ATOM(SYMBOL(name)) = args[0].clone() {
		match env.get(&name) {
			Ok(FUNCTION(SPECIAL(_))) | Ok(FUNCTION(NATIVE(_))) 
				=> return Err(OVERRIDE_RESERVED(name)),
			_ 	=> return Ok(env.set_bot(name, args[1].clone()))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "symbol"))
	}
}

pub fn gensym(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 0 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 0));
	}
	for num in 0.. {
		let sym = format!(":G{}", num);
		if let Err(..) = env.get(&sym) {
			return Ok(ATOM(SYMBOL(sym)));
		}
	}
	Err(NO_INPUT) //never reached
}

pub fn apply_lisp(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 2 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 2))
	} else if let FUNCTION(func) = args[0].clone() {
		if let LIST(lst) = args[1].clone() {
			apply(func, lst.get_items(), env)
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "list"))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "function"))
	}
}

pub fn eval_lisp(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else {
		eval(&args[0], env)
	}
}

fn macro_lisp(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	let lam = lambda(args, env);
	if let Ok(FUNCTION(LAMBDA(mac))) = lam {
		Ok(FUNCTION(MACRO(mac)))
	} else {
		lam //error
	}
}

fn defmacro(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 3 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 3));
	} 

	let mac = macro_lisp(vec!(args[1].clone(), args[2].clone()), env);
	if mac.is_err() {
		return mac;
	} else if let ATOM(SYMBOL(name)) = args[0].clone() {
		match env.get(&name) {
			Ok(FUNCTION(SPECIAL(_))) | Ok(FUNCTION(NATIVE(_))) 
				=> return Err(OVERRIDE_RESERVED(name)),
			_	=> return Ok(env.set(name, mac.ok().unwrap()))
		}
	} else {
		return Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "symbol"))
	}
}

fn macroexpand(args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let LIST(lst@CONS(..)) = args[0].clone() {
		let items = lst.get_items();
		let func = eval(&items[0], env);
		if func.is_err() {
			func
		} else if let Ok(FUNCTION(MACRO(mac))) = func {
			macroexpand_helper(&mac, tail(items), env)
		} else {
			Err(INVALID_ARGUMENT_TYPE(func.ok().unwrap(), "macro"))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "nonempty list"))
	}
}

fn macroexpand_helper(func: &Lambda, args: Vec<Datum>, env: &mut Env) -> Result<Datum, LispError> {
	if args.len() < func.args.len() {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), func.args.len()));
	}

	let mut params: Vec<Datum> = Vec::with_capacity(func.args.len());
	let mut optional_params: Vec<Datum> = Vec::with_capacity(func.optn.len());
	let mut key_params: Vec<(String, Datum)> = Vec::with_capacity(func.key.len());
	let mut rest_params: Vec<Datum> = Vec::new();

	for i in 0..func.args.len() {
		params.push(args[i].clone())
	}

	let mut is_key = false;
	let mut key_name = "";
	for i in func.args.len()..args.len() {
		if let ATOM(SYMBOL(ref name)) = args[i] {
			if name.starts_with(':') && i != args.len()-1 {
				let name = &name[1..];
				if func.contains_key(name.to_string()) && !is_key {
					if !key_params.clone().into_iter().any(|(key, _): (String,_)| key==":".to_string()+name) {
						key_name = &name;
						is_key = true;
						continue;
					}
				}
			} 
		}

		if is_key {
			key_params.push((key_name.to_string(), args[i].clone()));
			is_key = false;
		} else if optional_params.len() != func.optn.len() {
			optional_params.push(args[i].clone());
		} else if func.rest != None {
			rest_params.push(args[i].clone());
		} else {
			return Err(INVALID_NUMBER_OF_ARGS(args.len(), func.args.len()));
		}
	}

	env.push_map(&func.env);
	for (param, arg) in params.into_iter().zip(&func.args) {
		env.set(arg.clone(), param);
	}
	for (name, default) in func.optn.clone() {
		env.set(name.clone(), default);
	}
	for (param, arg) in optional_params.into_iter().zip(&func.optn) {
		env.set(arg.0.clone(), param);
	}
	for (name, default) in func.key.clone() {
		env.set(name.clone(), default);
	}
	for (name, val) in key_params {
		env.set(name, val);
	}
	if let Some(name) = func.rest.clone() {
		env.set(name.clone(), LIST(List::from_vec(rest_params)));
	}
	let res = eval(&func.body, env);
	env.pop();
	res
}