# Project Pathway



| Goal         | Description| Language Used |
|:-------------|:-----------|:--------------|
|Parse all simple expressions| The goal here is to make a simple parser to recognize all of the simple expressions of the language. This excludes function calls. We also make sure to keep track of line and column numbers.| Haskell|
|Bytecode generation for simple expression| Here we create an abstract machine that can compile our simple expression into bytecode|Haskell|
|Generate Simple Binary File| Here we create a simple binary file that holds the bytecode in it created from our previous goal| Haskell|
|Read binary file| This is where we read in binary data and write a way to convert it into bytecode data structure on demand| Rust|
|Setup Cranelift| This is where we convert our bytecode into Cranelift IR to allow for JIT compilation| Rust|
|JIT Compilation and Execution| This is where we actually compile our Cranelift IR and run the generated code.| Rust|
|Adding Variables and Typechecking| Here we adding in variables, and typechecking variables to ensure they are the right type| Haskell|
|SSA| We convert our AST into a Single Static Assignment form and use that for the rest of the compiler.| Haskell|
|JITing Variables| Here we JIT our variables| Rust|
|Adding Loop Constructs| We now add for, while, and loop loops to our parser along with labeled breaks and continues.| Haskell|
|Compiling Loop Constructs| We now compile our loop constructs into their bytecode representation and extend our binary file to support them as well.| Haskell|
|Setting up Cranelift for loops| Here we figure out how to make our loop bytecode into Cranelift IR that we can compile.| Rust|
|Parsing and Compiling Block Expressions| Here we parse and compile block expressions which will be useful later on when we declare functions| Haskell|
|Adding Block Expressions to JIT| Here we add compilation of block expressions to our interpreter| Rust|
|Adding If Statements and Expressions| Here we parse in if statements and expressions to our language.| Haskell|
|Compilation of If Statements and Expressions| We extend our compiler to support if statements/expressions| Haskell|
|JIT Compilation of If Statements and Expressions| We extend our Cranelift IR generator to support If constructions| Rust|
|Parsing of Function Definitions| Here we use our block parser from BLock Expressions to generate functions to call| Haskell|
|Compilation of Functions into Bytecode| Here we compile our functions into bytecode.| Haskell|
|Outputing of Binary `.module` Files| Here we change from our simple binary output file to a more complicated module file which contains much more data in it. However, not all of it will be implemented yet| Haskell|
|Reading in of Binary `.module` Files| Here we change our byte parser to read in our module files| Rust|
|JIT Compiling of Module Files| Here we convert our functions into Cranelift IR functions so that we can execute them| Rust|
|Calling Main| Here we set up calling main| Rust|
|Function Call Parsing and Compiling| Here we extend the parser and compiler to call functions.| Haskell|
|**Hello World**| Here we finally enable the VM to call a Native Function that prints Hello World| Rust|
