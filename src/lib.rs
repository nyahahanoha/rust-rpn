use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct RPN {
    stack: Vec<f32>,
    operator: Operation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    NONE,
}

pub fn new(v: Vec<f32>) -> RPN {
    RPN {
        stack: v,
        operator: Operation::NONE,
    }
}

impl RPN {
    fn pop(&mut self) -> Result<f32, String> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None => Err("There is no value in stack".to_string()),
        }
    }

    fn push(&mut self, n: f32) {
        self.stack.push(n);
    }

    fn delete_operator(&mut self) {
        self.operator = Operation::NONE;
    }

    fn set_operator(&mut self, o: Operation) {
        if self.is_none_operator() {
            self.operator = o;
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

    fn execute<'a>(&mut self) -> Result<f32, String> {
        if self.is_none_operator() {
            return Err("Not set operator".to_string());
        } else if self.len() < 2 {
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
            ans = b / a;
        } else {
            panic!("Dont'know this operator");
        }
        self.push(ans);
        self.delete_operator();
        return Ok(ans);
    }
}

impl Display for RPN {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.stack)
    }
}

#[cfg(test)]
mod tests {
    use crate::Operation;
    use crate::RPN;
    use rstest::*;
    #[fixture]
    pub fn fixture() -> RPN {
        RPN {
            stack: vec![1.0, 3.0, 5.0],
            operator: Operation::NONE,
        }
    }
    #[rstest]
    fn test_init(fixture: RPN) {
        assert_eq!(
            fixture,
            RPN {
                stack: vec![1.0, 3.0, 5.0],
                operator: Operation::NONE,
            }
        );
        assert_ne!(
            fixture,
            RPN {
                stack: vec![3.0],
                operator: Operation::ADD,
            }
        );
    }
    #[rstest]
    fn test_pop(fixture: RPN) {
        let mut a = fixture.clone();
        assert_eq!(a.pop(), Ok(5.0));
        assert_eq!(a.pop(), Ok(3.0));
        assert_eq!(a.pop(), Ok(1.0));
        assert_eq!(a.pop(), Err("There is no value in stack".to_string()));
    }
    #[rstest]
    fn test_push(fixture: RPN) {
        let mut a = fixture.clone();
        a.push(2.0);
        assert_eq!(a.to_string(), "[1.0, 3.0, 5.0, 2.0]");
        a.push(4.0);
        assert_eq!(a.to_string(), "[1.0, 3.0, 5.0, 2.0, 4.0]");
    }
    #[rstest]
    fn test_change_operator(fixture: RPN) {
        let mut a = fixture.clone();
        assert_eq!(a.is_none_operator(), true);
        a.set_operator(Operation::ADD);
        assert_eq!(a.operator, Operation::ADD);
        assert_eq!(a.is_none_operator(), false);
        a.delete_operator();
        assert_eq!(a.operator, Operation::NONE);
    }
    #[rstest]
    fn test_execute(fixture: RPN) {
        let mut a = fixture.clone();
        a.set_operator(Operation::ADD);
        assert_eq!(a.execute(), Ok(8.0));
        assert_eq!(a.to_string(), "[1.0, 8.0]");
        assert_eq!(a.operator, Operation::NONE);
        a.set_operator(Operation::SUB);
        assert_eq!(a.execute(), Ok(-7.0));
        assert_eq!(a.to_string(), "[-7.0]");
        a.push(9.0);
        a.set_operator(Operation::MUL);
        assert_eq!(a.execute(), Ok(-63.0));
        assert_eq!(a.to_string(), "[-63.0]");
        a.push(-3.0);
        assert_eq!(a.to_string(), "[-63.0, -3.0]");
        a.set_operator(Operation::DIV);
        assert_eq!(a.execute(), Ok(21.0));
        assert_eq!(a.to_string(), "[21.0]");
        assert_eq!(a.execute(), Err("Not set operator".to_string()));
        a.set_operator(Operation::ADD);
        assert_eq!(a.execute(), Err("Stack is shortage".to_string()));
    }
}
