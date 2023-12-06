use std::io;
use RPN::*;

fn main() {
    let mut rpn = RPN::new(Vec::new());
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        let v: Vec<&str> = buffer.split_whitespace().collect::<Vec<&str>>();
        for i in v.iter() {
            match i.parse::<f32>() {
                Ok(n) => rpn.push(n),
                Err(_) => match i.parse::<char>() {
                    Ok(c) => execute(c, &mut rpn),
                    Err(_) => println!("Don't know this operation: {}", i),
                },
            }
        }
        println!("{}", rpn);
    }
}

fn execute(c: char, rpn: &mut Rpn) {
    if c == '+' {
        rpn.set_operator('+');
    } else if c == '-' {
        rpn.set_operator('-');
    } else if c == '*' {
        rpn.set_operator('*');
    } else if c == '/' {
        rpn.set_operator('/');
    } else {
        println!("Don't know this operation: {}", c);
    }
    let _ = rpn.execute();
}
