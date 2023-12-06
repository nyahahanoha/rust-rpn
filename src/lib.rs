use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Rpn {
    stack: Vec<f32>,
    operator: Operation,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    NONE,
}
pub fn new(v: Vec<f32>) -> Rpn {
    Rpn {
        stack: v,
        operator: Operation::NONE,
    }
}

impl Rpn {
    pub fn pop(&mut self) -> Result<f32, String> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None => Err("There is no value in stack".to_string()),
        }
    }

    pub fn push(&mut self, n: f32) {
        self.stack.push(n);
    }

    fn delete_operator(&mut self) {
        self.operator = Operation::NONE;
    }

    pub fn set_operator(&mut self, o: char) {
        if self.is_none_operator() {
            if o == '+' {
                self.operator = Operation::ADD;
            } else if o == '-' {
                self.operator = Operation::SUB;
            } else if o == '*' {
                self.operator = Operation::MUL;
            } else if o == '/' {
                self.operator = Operation::DIV;
            } else {
                panic!("Don't know this Operation");
            }
        } else {
            panic!("Operator has been set to Operation");
        }
    }

    fn is_none_operator(&mut self) -> bool {
        if self.operator == Operation::NONE {
            return true;
        } else {
            return false;
        }
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn execute<'a>(&mut self) -> Result<f32, String> {
        if self.is_none_operator() {
            panic!("Not set operator");
        } else if self.len() < 2 {
            self.delete_operator();
            return Err("Stack is shortage".to_string());
        }
        let a: f32 = self.pop()?;
        let b: f32 = self.pop()?;
        let ans: f32;
        if self.operator == Operation::ADD {
            ans = b + a;
        } else if self.operator == Operation::SUB {
            ans = b - a;
        } else if self.operator == Operation::MUL {
            ans = b * a;
        } else if self.operator == Operation::DIV {
            if a == 0.0 {
                self.push(b);
                self.push(a);
                self.delete_operator();
                return Err("Cannot be divided by zero".to_string());
            }
            ans = b / a;
        } else {
            panic!("Dont'know this operator");
        }
        self.push(ans);
        self.delete_operator();
        return Ok(ans);
    }
}

impl Display for Rpn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.stack)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::*;
    #[rstest]
    fn test_init() {
        assert_eq!(
            new(vec![1.0, 3.0, 5.0]),
            Rpn {
                stack: vec![1.0, 3.0, 5.0],
                operator: Operation::NONE,
            }
        );
    }
    #[fixture]
    pub fn fixture() -> Rpn {
        new(vec![1.0, 3.0, 5.0])
    }
    #[rstest]
    fn test_pop(fixture: Rpn) {
        let mut a = fixture.clone();
        assert_eq!(a.pop(), Ok(5.0));
        assert_eq!(a.pop(), Ok(3.0));
        assert_eq!(a.pop(), Ok(1.0));
        assert_eq!(a.pop(), Err("There is no value in stack".to_string()));
    }
    #[rstest]
    fn test_push(fixture: Rpn) {
        let mut a = fixture.clone();
        a.push(2.0);
        assert_eq!(a.to_string(), "[1.0, 3.0, 5.0, 2.0]");
        a.push(4.0);
        assert_eq!(a.to_string(), "[1.0, 3.0, 5.0, 2.0, 4.0]");
    }
    #[rstest]
    fn test_change_operator(fixture: Rpn) {
        let mut a = fixture.clone();
        assert_eq!(a.is_none_operator(), true);
        a.set_operator('+');
        assert_eq!(a.operator, Operation::ADD);
        assert_eq!(a.is_none_operator(), false);
        a.delete_operator();
        assert_eq!(a.operator, Operation::NONE);
        a.set_operator('-');
        assert_eq!(a.operator, Operation::SUB);
        a.delete_operator();
        a.set_operator('*');
        assert_eq!(a.operator, Operation::MUL);
        a.delete_operator();
        a.set_operator('/');
        assert_eq!(a.operator, Operation::DIV);
    }
    #[rstest]
    fn test_execute(fixture: Rpn) {
        let mut a = fixture.clone();
        a.set_operator('+');
        assert_eq!(a.execute(), Ok(8.0));
        assert_eq!(a.to_string(), "[1.0, 8.0]");
        assert_eq!(a.operator, Operation::NONE);
        a.set_operator('-');
        assert_eq!(a.execute(), Ok(-7.0));
        assert_eq!(a.to_string(), "[-7.0]");
        a.push(9.0);
        a.set_operator('*');
        assert_eq!(a.execute(), Ok(-63.0));
        assert_eq!(a.to_string(), "[-63.0]");
        a.push(-3.0);
        assert_eq!(a.to_string(), "[-63.0, -3.0]");
        a.set_operator('/');
        assert_eq!(a.execute(), Ok(21.0));
        assert_eq!(a.to_string(), "[21.0]");
        a.set_operator('+');
        assert_eq!(a.execute(), Err("Stack is shortage".to_string()));
        a.push(0.0);
        a.set_operator('/');
        assert_eq!(a.execute(), Err("Cannot be divided by zero".to_string()));
    }
    #[rstest]
    #[should_panic]
    fn test_panic(fixture: Rpn) {
        let mut a = fixture.clone();
        a.set_operator('&');
        a.set_operator('+');
        a.set_operator('+');
    }
}
