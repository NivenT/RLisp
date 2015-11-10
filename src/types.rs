use std::fmt;
use std::ops;
use std::cmp;

use std::collections::HashMap;

pub fn tail<T>(vec: Vec<T>) -> Vec<T> {
	vec.into_iter().skip(1).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Datum {
	ATOM(Atom),
	LIST(List),
	FUNCTION(Function)
}

use self::Datum::*;

impl fmt::Display for Datum {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			FUNCTION(ref a)	=> write!(f, "{}", a),
			ATOM(ref a)		=> write!(f, "{}", a),
			LIST(ref a)		=> write!(f, "{}", a)
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
	RATIONAL(i64,i64),
	INTEGER(i64),
	REAL(f64)
}

use self::Number::*;

impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			RATIONAL(ref a, ref b)	=> write!(f, "{}/{}", a, b),
			INTEGER(ref a)			=> write!(f, "{}", a),
			REAL(ref a)				=> write!(f, "{}", a)
		}
	}
}

impl ops::Add for Number {
	type Output = Number;
	fn add(self, rhs: Number) -> Number {
		match self {
			RATIONAL(a,b) => {
				match rhs {
					RATIONAL(c,d)	=> RATIONAL(a*d+b*c,b*d),
					INTEGER(c) 		=> RATIONAL(a+c*b,b),
					REAL(c)			=> REAL(a as f64/b as f64+c)
				}
			}
			INTEGER(a) => {
				match rhs {
					RATIONAL(b,c)	=> RATIONAL(a*c+b,c),
					INTEGER(b) 		=> INTEGER(a+b),
					REAL(b)			=> REAL(a as f64+b)
				}
			},
			REAL(a) => {
				match rhs {
					RATIONAL(b,c)	=> REAL(a+b as f64/c as f64),
					INTEGER(b)		=> REAL(a+b as f64),
					REAL(b)			=> REAL(a+b)
				}
			}
		}
	}
}

impl ops::Sub for Number {
	type Output = Number;
	fn sub(self, rhs: Number) -> Number {
		match self {
			RATIONAL(a,b) => {
				match rhs {
					RATIONAL(c,d)	=> RATIONAL(a*d-b*c,b*d),
					INTEGER(c) 		=> RATIONAL(a-c*b,b),
					REAL(c)			=> REAL(a as f64/b as f64-c)
				}
			}
			INTEGER(a) => {
				match rhs {
					RATIONAL(b,c)	=> RATIONAL(a*c-b,c),
					INTEGER(b) 		=> INTEGER(a-b),
					REAL(b)			=> REAL(a as f64-b)
				}
			},
			REAL(a) => {
				match rhs {
					RATIONAL(b,c)	=> REAL(a-b as f64/c as f64),
					INTEGER(b)		=> REAL(a-b as f64),
					REAL(b)			=> REAL(a-b)
				}
			}
		}
	}
}

impl ops::Mul for Number {
	type Output = Number;
	fn mul(self, rhs: Number) -> Number {
		match self {
			RATIONAL(a,b) => {
				match rhs {
					RATIONAL(c,d)	=> RATIONAL(a*c,b*d),
					INTEGER(c) 		=> RATIONAL(a*c,b),
					REAL(c)			=> REAL(a as f64/b as f64*c)
				}
			}
			INTEGER(a) => {
				match rhs {
					RATIONAL(b,c)	=> RATIONAL(a*b,c),
					INTEGER(b) 		=> INTEGER(a*b),
					REAL(b)			=> REAL(a as f64*b)
				}
			},
			REAL(a) => {
				match rhs {
					RATIONAL(b,c)	=> REAL(a*b as f64/c as f64),
					INTEGER(b)		=> REAL(a*b as f64),
					REAL(b)			=> REAL(a*b)
				}
			}
		}
	}
}

impl ops::Div for Number {
	type Output = Number;
	fn div(self, rhs: Number) -> Number {
		match self {
			RATIONAL(a,b) => {
				match rhs {
					RATIONAL(c,d)	=> RATIONAL(a*d,b*c),
					INTEGER(c) 		=> RATIONAL(a,b*c),
					REAL(c)			=> REAL(a as f64/b as f64/c)
				}
			}
			INTEGER(a) => {
				match rhs {
					RATIONAL(b,c)	=> RATIONAL(a*c,b),
					INTEGER(b) 		=> RATIONAL(a,b),
					REAL(b)			=> REAL(a as f64/b)
				}
			},
			REAL(a) => {
				match rhs {
					RATIONAL(b,c)	=> REAL(a/(b as f64/c as f64)),
					INTEGER(b)		=> REAL(a/b as f64),
					REAL(b)			=> REAL(a/b)
				}
			}
		}
	}
}

impl ops::Neg for Number {
	type Output = Number;
	fn neg(self) -> Number {
		match self {
			RATIONAL(a,b) 	=> RATIONAL(-a,b),
			INTEGER(a)		=> INTEGER(-a),
			REAL(a)			=> REAL(-a)
		}
	}
}

impl cmp::PartialOrd for Number {
	fn partial_cmp(&self, rhs: &Number) -> Option<cmp::Ordering> {
		match *self {
			RATIONAL(a,b) => {
				match *rhs {
					RATIONAL(c,d)	=> (a*d).partial_cmp(&(c*b)),
					INTEGER(c) 		=> a.partial_cmp(&(c*b)),
					REAL(c)			=> (a as f64/b as f64).partial_cmp(&c)
				}
			}
			INTEGER(a) => {
				match *rhs {
					RATIONAL(b,c)	=> (a*c).partial_cmp(&b),
					INTEGER(b) 		=> a.partial_cmp(&b),
					REAL(b)			=> (a as f64).partial_cmp(&b)
				}
			},
			REAL(a) => {
				match *rhs {
					RATIONAL(b,c)	=> a.partial_cmp(&(b as f64/c as f64)),
					INTEGER(b)		=> a.partial_cmp(&(b as f64)),
					REAL(b)			=> a.partial_cmp(&b)
				}
			}
		}
	}
}

fn gcd(a: i64, b: i64) -> i64 {
	if b > a {
		gcd(b, a)
	} else if b == 0 {
		a
	} else {
		gcd(b, a%b)
	}
}

use std::i64;

impl Number {
	pub fn simplify(&self) -> Number {
		match *self {
			RATIONAL(a,b) if b < 0 => RATIONAL(-a,-b).simplify(),
			RATIONAL(a,b) if a < 0 => -RATIONAL(-a,b).simplify(),
			RATIONAL(a,b) 	=> {
				if a%b == 0 {
					INTEGER(a/b)
				} else {
					let d = gcd(a,b);
					RATIONAL(a/d, b/d)
				}
			},
			REAL(a) if a == a.floor() && a <= i64::MAX as f64 && a >= i64::MIN as f64 => 
				INTEGER(a as i64),
			e @ _			=> e
		}
	}
	pub fn val(&self) -> f64 {
		match *self {
			RATIONAL(a,b)	=> a as f64/b as f64,
			INTEGER(a)		=> a as f64,
			REAL(a)			=> a
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
	SYMBOL(String),
	STRING(String),
	NUMBER(Number),
	T
}

use self::Atom::*;

impl fmt::Display for Atom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			SYMBOL(ref a)	=> write!(f, "{}", a),
			STRING(ref a)	=> write!(f, "\"{}\"", a),
			NUMBER(ref a)	=> write!(f, "{}", a),
			T 				=> write!(f, "T")
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum List {
	CONS(Box<Datum>,Box<Datum>),
	NIL
}

use self::List::*;

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			NIL 		=> write!(f, "NIL"),
			CONS(..)	=> write!(f, "{}", self.list_print())
		}
	}
}

impl List {
	pub fn from_vec(items: Vec<Datum>) -> List {
		if items.is_empty() {
			NIL
		} else {
			CONS(Box::new(items[0].clone()), Box::new(LIST(List::from_vec(
												tail(items)))))
		}
	}

	pub fn get_items(&self) -> Vec<Datum> {
		match *self {
			CONS(ref l, ref r)	=> {
				let mut ret = vec![*l.clone()];
				match **r {
					LIST(ref a) => {ret.extend(a.get_items())},
					_			=> {ret.push(*r.clone())}
				}
				ret
			},
			NIL 				=> vec![]
		}
	}
	
	fn last(&self) -> Datum {
		match *self {
			CONS(_, ref r)	=> match **r {
				LIST(ref l)	=> l.last(),
				ref e @ _	=> e.clone()
			},
			NIL						=> LIST(NIL)
		}
	}

	fn list_print(&self) -> String {
		let mut ret = "(".to_string();
		let items = self.get_items();
		for i in 0..items.len()-1 {
			ret = format!("{}{} ", ret, items[i]);
		}
		if self.last() == LIST(NIL) {
			ret = format!("{}{})", ret, *items.last().unwrap());
		} else {
			ret = format!("{}. {})", ret, *items.last().unwrap());
		}
		ret
	}

	pub fn car(&self) -> Datum {
		match *self {
			CONS(ref c, _)	=> *c.clone(),
			NIL 			=> LIST(NIL)
		}
	}

	pub fn cdr(&self) -> Datum {
		match *self {
			CONS(_, ref c)  => *c.clone(),
			NIL  			=> LIST(NIL)
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Native {
	ADD, SUB, MUL, DIV, MOD, POWI, POWR,
	GT, GE, LT, LE, MATH_EQ,
	LIST_FUNC, CAR, CDR, CONS_FUNC, 
	NTH, NTH_CDR, MOST,
	LOAD,
	FLOOR, CEIL,
	TYPE,
	IS_ATOM, IS_LIST, IS_CONS, IS_SYMBOL,
	EQUAL,
	WRITE_TO_STRING, READ_FROM_STRING, STRING_CONCAT, PRINT,
	NOT,
	SET,
	GENSYM,
	APPLY,
	EVAL,
	RANDINT, RANDBOOL, RANDREAL
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Special {
	IF,
	LET, LET_STAR,
	PROGN,
	QUOTE, BACKQUOTE,
	DEFINE, DEFUN, DEFMACRO,
	LAMBDA_FUNC, MACRO_FUNC,
	MACROEXPAND,
	TIME
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lambda {
	pub args: Vec<String>,
	pub optn: Vec<(String, Datum)>,
	pub key:  Vec<(String, Datum)>,
	pub rest: Option<String>,
	pub body: Box<Datum>,
	pub env:  HashMap<String, Datum>
}

fn to_string(v: Vec<(String, Datum)>) -> String {
	if v.is_empty() {
		return "[]".to_string();
	}
	let mut ret = "[".to_string();
	for i in 0..v.len()-1 {
		ret = format!("{}({:?} {}), ", ret, v[i].0, v[i].1);
	}
	ret = format!("{}({:?} {})", ret, v.last().unwrap().0, v.last().unwrap().1);
	format!("{}]", ret)
}

impl fmt::Display for Lambda {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = format!("{{");
		if !self.args.is_empty() {
			s = format!("{}args: {:?}, ", s, self.args.clone())
		}
		if !self.optn.is_empty() {
			s = format!("{}optional args: {}, ", 
				s, to_string(self.optn.clone()))
		}
		if !self.key.is_empty() {
			s = format!("{}key args: {}, ", 
				s, to_string(self.key.clone()))
		}
		if self.rest != None {
			s = format!("{}rest: {:?}, ", s, self.rest.clone().unwrap())
		}
		write!(f, "{}body: {}}}", s, self.body.clone())
	}
}

impl Lambda {
	pub fn contains_key(&self, name: String) -> bool {
	    self.key.clone().into_iter().map(|keyval| keyval.0)
	    .collect::<Vec<String>>().contains(&name)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
	SPECIAL(Special),
	NATIVE(Native),
	LAMBDA(Lambda),
	MACRO(Lambda)
}

use self::Function::*;

impl fmt::Display for Function {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			SPECIAL(s)		=> write!(f, "{:?}", s),
			NATIVE(n)		=> write!(f, "{:?}", n),
			LAMBDA(ref l)	=> write!(f, "Lambda{}", l),
			MACRO(ref m)	=> write!(f, "Macro{}", m)
		}
	}
}