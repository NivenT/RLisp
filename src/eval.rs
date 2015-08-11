use errors::*;
use types::*;
use env::*;

use errors::LispError::*;
use types::Datum::*;
use types::List::*;
use types::Atom::*;

pub fn eval(form: Datum, env: &mut Env) -> Result<Datum, LispError> {
	match form {
		LIST(ref l)	=> eval_list(l, env),
		ATOM(ref a) => eval_atom(a, env),
		_			=> Err(LispError::INVALID_FORM)
	}
}

fn eval_list(form: &List, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		CONS(ref car, ref cdr) 	=> {
			let f = eval(*car.clone(), env);
			if f.is_err() {
				return f;
			} match f.ok().unwrap() {
				FUNCTION(func) 	=> {
					match *cdr.clone() {
						LIST(list) 	=> {
							let mut args: Vec<Datum> = vec![];
							for item in list.get_items() {
								let res = eval(item, env);
								if res.is_err() {
									return res;
								} else {
									args.push(res.ok().unwrap());
								}
							}

							env.apply(func, List::from_vec(args))
						},
						_		=> Err(INVALID_ARG_LIST)
					}
				},
				_				=> Err(UNKNOWN_FUNCTION)
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