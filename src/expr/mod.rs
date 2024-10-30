use std::any::Any;
use super::scanner::token::Token;

type Object = Box<dyn Any>;
trait Expr {}
struct Binary {
	pub left: Box<dyn Expr>,
	pub operator: Token,
	pub right: Box<dyn Expr>,
}
impl Expr for Binary {}
impl Binary {
	fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>, ) -> Box<Self> {
		Box::new(Binary {left,operator,right, })
	}
}

struct Grouping {
	pub expression: Box<dyn Expr>,
}
impl Expr for Grouping {}
impl Grouping {
	fn new(expression: Box<dyn Expr>, ) -> Box<Self> {
		Box::new(Grouping {expression, })
	}
}

struct Literal {
	pub value: Object,
}
impl Expr for Literal {}
impl Literal {
	fn new(value: Object, ) -> Box<Self> {
		Box::new(Literal {value, })
	}
}

struct Unary {
	pub operator: Token,
	pub right: Box<dyn Expr>,
}
impl Expr for Unary {}
impl Unary {
	fn new(operator: Token, right: Box<dyn Expr>, ) -> Box<Self> {
		Box::new(Unary {operator,right, })
	}
}

