use std::collections::HashMap;

pub enum Expression {
    Add(Vec<Expression>),
    Minus(Vec<Expression>),
    Multiply(Vec<Expression>),
    Variable(String), //this could resolve to something else
    Number(f64)
}

fn evaluate(expression : &Expression, environment: &Environment) -> f64{
    match expression {
        Expression::Number(value) => *value,
        Expression::Add(_) => evaluate_addition(expression, environment),
        Expression::Minus(_) => evaluate_minusition(expression, environment),
        Expression::Variable(name) => evaluate(&environment.value_for_key(&name), environment),
        _ => -1.0    
    }
}

fn main() { 
    let addition = Expression::Add(vec![Expression::Number(2.0), Expression::Multiply(vec![Expression::Number(2.0), Expression::Minus(vec![Expression::Number(4.0), Expression::Number(2.0)])])]);
    print_expression(&addition);
}

pub struct Environment {
    dict: HashMap<String,Expression>
}

impl Environment {
    fn new()-> Environment { 
        let mut env = Environment {        
            dict: { crate::HashMap::new()},
        };
        env.dict.insert(String::from(""), crate::Expression::Number(0.0));
        env
    }

    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        if self.dict.contains_key(key) {
            &self.dict[key]
        } else {
            panic!("no value for key found in environment")
        }
    }
}
fn print_expression(expression: &Expression) {
    print_expression_h(expression);
    println!();
}

fn print_expression_h(expression: &Expression) {
    if let Expression::Add(expressions) = expression {
        let iter = expressions.iter();
        print!("(+");
        iter.fold((), |_total, next| {print_expression_h(next);});
        print!(")")
    } else if let Expression::Multiply(expressions) = expression {
        print!("(*");
        let iter = expressions.iter();
        iter.fold((), |_total, next| {print_expression_h(next);});
        print!(")")
    } else if let Expression::Minus(expressions) = expression {
        let iter = expressions.iter();
        print!("(-");
        iter.fold((), |_total, next| {print_expression_h(next);});
        print!(")");
    } else if let Expression::Number(expressions) = expression {
        print!(" {} ", evaluate(expression, &Environment::new()));
    } 
}

#[cfg(test)]
mod testing {

    fn num(value : f64)-> crate::Expression {
        crate::Expression::Number(value)
    }

    #[test]
    fn test_bootstrap() {
        assert_eq!(0,0);
    }

    #[test]
    fn test_evaluate_number() {
        let number = num(42.0);
        let value = crate::evaluate(&number, &crate::Environment::new());
        assert_eq!(value, 42.0);
    }

    #[test]
    fn test_environment_contains_variable() {
        //arrange
        let mut environment = crate::Environment {dict: crate::HashMap::new()};
        environment.dict.insert(String::from("foo"), num(5.0));
        //act
        // let key = environment.key;
        let value= crate::evaluate(&environment.dict["foo"], &environment);

        //assert
        assert_eq!(value, 5.0);
    }

    #[test]
    fn test_empty_environment() {
        //arrange
        let environment = crate::Environment::new();
        //act
        // let key = environment.key;
        let value = crate::evaluate(&environment.dict[""], &environment);
        //assert
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_environment_holds_value() {
        //arrange
        let mut environment = crate::Environment {dict: crate::HashMap::new()};
        environment.dict.insert(String::from("foo"), num(42.0));
        //act
        let expression = environment.value_for_key(&String::from("foo"));
        let value = crate::evaluate(&expression, &environment);
        //assert
        assert_eq!(value, 42.0);
    } 

    #[test]
    fn test_addition() {
        //arranage 
        let addition = crate::Expression::Add(vec![num(2.0), num(2.0)]);
        let mut environment = crate::Environment {dict: crate::HashMap::new()};
        environment.dict.insert(String::from("foo"), num(42.0));
        //act
        let value = crate::evaluate_addition(&addition, &environment);
        assert_eq!(value, 4.0);
    }

    #[test]
    fn test_minusition() {
        //arranage 
        let addition = crate::Expression::Minus(vec![num(2.0), num(2.0)]);
        let mut environment = crate::Environment {dict: crate::HashMap::new()};
        environment.dict.insert(String::from("foo"), num(42.0));
        //act
        let value = crate::evaluate_minusition(&addition, &environment);
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_variable() {
        let variable = crate::Expression::Variable(String::from("foo"));
        let mut environment = crate::Environment{dict: crate::HashMap::new()};
        environment.dict.insert(String::from("foo"), num(42.0));
        crate::evaluate(&variable, &environment);
    }
    #[test]
    fn test_dict(){
        let variable = crate::Expression::Variable(String::from("Square2"));
        let mut environment = crate::Environment{
            dict: 
            crate::HashMap::new() 
        };
        environment.dict.insert(String::from("Square2"), crate::Expression::Multiply(vec![num(2.0), num(2.0)]));
    }
}

fn evaluate_addition(add: &Expression, environment: &Environment) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| {total + evaluate(next, environment)})
    } else {
        panic!("expected addition")
    }
}

fn evaluate_minusition(minus: &Expression, environment: &Environment) -> f64 {
    if let Expression::Minus(expressions) = minus {
        let iter = expressions.iter();
        iter.fold(-2.0*(evaluate(&expressions[0], &Environment::new())), |total, next| {total + evaluate(next, environment)})
    } else {
        panic!("expected minus")
    }
}
