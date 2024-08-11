# Statements and Expressions

### Statements
|Statemement        |Example                       |     Description      |
|:------------------|:-----------------------------|:---------------------|
|Expression         | `1 + 3;`                     |contains an expression. Outputs the Unit type|
|Hanging Expression | `1 + 3`                      |contains an expression. Outputs the type of the expression|
|Let                | `let x = 4;`                 |takes a pattern and an expression. Binds a variable|
|While Loop         | <pre>while x < 3 { <br> &emsp;// Do Something </br> <br>}</br></pre>|Condition based loop structure|
|For Loop           | <pre>for i in 0..3 { <br> &emsp;// Do Something </br> <br>}</br></pre>|Iteration based loop structure. Takes a pattern to bind to and an iterable|
|With               | <pre>with mutex.lock() as mutexLock { <br> &emsp;// Do Something </br> <br>}</br></pre>|RAII emulation. Takes a value that implements the `Resource` trait and binds it.|
|Labled             |<pre>'a: while x < 3 { <br> &emsp;//Do Something </br> <br>}</br></pre>|Labled statement. This is to allow for breaking or continuing loops|

### Expressions
|Expression         |Example                       |    Description             |
|:------------------|:-----------------------------|:---------------------------|
|Literal            |`3`                           |Simple literal expression, can be an int, float, char, or str|
|Varible            |`x`                           |Simple variable expression, is a bound variable|
|Add                |`1 + 2`                       |Simple add expression. Can be overloaded with `std::ops::Add` trait|
|Subtract           |`1 - 2`                       |Simple subtraction expression. Can be overloaded with `std::ops::Sub` trait|
|Multiplication     |`1 * 2`                       |Simple multiplication expression. Can be overloaded with `std::ops::Mul` trait|
|Division           |`1 / 2`                       |Simple division expression. Can be overloaded with `std::ops::Div` trait|
|Modulo             |`1 % 2`                       |Simple modulo expression. Can be overloaded with `std::ops::Mod` trait|
|Logical And        |`x && x < 3`                  |Simple logical and expression. Cannot be overloaded.|
|Logical Or         |`x \|\| x < 3`                |Simple logical or expression. Cannot be overloaded.|
|Equal Expression   |`x == y`                      |Simple equal expression. Can be overloaded with `std::ops::Eq` trait|
|Not Equal Expression|`x != y`                     |Simple not equal expression. Is overloaded with `std::ops::Eq` trait|
|Bitwise And        |`x & y`                       |Simple bitwise and expression. Is overloaded with `std::ops::BitAnd` trait|
|Bitwise Or         |`x \| y`                       |Simple bitwise or expression. Is overloaded with `std::ops::BitOr` trait|
|Bitwise Xor        |`x ^ y`                       |Simple bitwise xor expression. Is overloaded with `std::ops::BitXor` trait|
|Left Shift         |`x << y`                      |Bitwise Left shift expression. Is overloaded with `std::ops::LShift` trait|
|Right Shift        |`x >> y`                      |Bitwise Right shift expression. Is overloaded with `std::ops::RShift` trait|
|Negation           |`-x`                          |Numerical negation. Is overloaded with `std::ops::Neg` trait|
|Bitwise Not        |`!x`                          |Bitwise not. Is overloaded with `std::ops::Not` trait|
|Exclusive Range    |`0..4`                        |Is a range of values. If an int or char it gets compiled out, otherwise it can be overloaded with `std::ops::ExRange` trait|
|Inclusive Range    |`0..=4`                       |Is a range of values. If an int or char it gets compiled out, otherwise it can be overloaded with `std::ops::InRange` trait|
|Greater Than       |`x > y`                       |Comparison. Can be overloaded with `std::ops::Ord`|
|Less Than          |`x < y`                       |Comparison. Can be overloaded with `std::ops::Ord`|
|Greater Than or Equal|`x >= y`                       |Comparison. Can be overloaded with `std::ops::Ord`|
|Less Than or Equal  |`x <= y`                       |Comparison. Can be overloaded with `std::ops::Ord`|
|Return             |`return 4`                    |Returns from current function with a value or without a value|
|Break              |`break 'a 3`                  |Breaks either the current loop or a labeled loop. If a loop expression then it may return a value|
|Continue           |`continue 'a`                 |Continues either the current loop or a labeled loop.|
|Loop               |<pre>loop { <br> &emsp;//Do Something </br> <br>}</br></pre>|Infinte loop. If used with break expression it may return a value otherwise unit|
|Try                |`x?`                          |Returns the current function if the datatype is a certain datatype, otherwise it provides a value. This is like how Haskell's `do` notation works. Can be overloaded with `std::ops::Try`|
|Field Access       |`x.x`                         |Accesses a field from an object. Depending on left or right hand side, this either gets or sets the value.|
|Call               |`foo(x, 3)`                   |Calls a function with arguments. Can be used with Field Access to call a method.|


