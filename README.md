# rust-rpn

## How to do
We can used RPN Calculator with Rust.

## example
```
10 1 2 + 3 2 - + - \n
[6.0]

2 \n
[6.0, 2.0]

/ \n
[3.0]

0.5 / \n
[6.0]

0 / \n
[6.0, 0.0]

* \n
[0.0]

3.0 - \n
[-3.0]

-1.0 + \n
[-4.0]
```

If division by zero occurs, the stack retains a value of zero, and the operation is not executed.
