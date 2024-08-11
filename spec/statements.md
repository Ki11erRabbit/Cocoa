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

### Expressions
|Expression         |Example                       |    Description             |
|:------------------|:-----------------------------|:---------------------------|
|Add                |`1 + 2`                       |Simple add expression. Can be overloaded with `std::ops::Add` trait|
|Subtract           |`1 - 2`                       |Simple subtraction expression. Can be overloaded with `std::ops::Sub` trait|
|Multiplication     |`1 * 2`                       |Simple multiplication expression. Can be overloaded with `std::ops::Mul` trait|
|Division           |`1 / 2`                       |Simple division expression. Can be overloaded with `std::ops::Div` trait|
|Modulo             |`1 % 2`                       |Simple modulo expression. Can be overloaded with `std::ops::Mod` trait|
|Logical And        |`x && x < 3`                  |Simple logical and expression. Cannot be overloaded.|
|Logical Or         |`x \|\| x < 3`                |Simple logical or expression. Cannot be overloaded.|
|Equal Expression   |`x == y`                      |Simple equal expression. Can be overloaded with `std::ops::Eq` trait|
