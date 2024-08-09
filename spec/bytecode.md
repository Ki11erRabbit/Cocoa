# Bytecode Definition

### Type conversion
|Alias | Type|
|:----:|:---:|
|PoolPointer| u64 |
| PoolValue| u8,i8,u16,i16,u32,i32,u64,i64 |

### Bytecode Definition
| Operation | Operands   |  Output | Description |
|:---------:|:----------:|:-------:|:------------|
|LoadConstant|PoolPointer|PoolValue|Takes a value from the constant pool and puts it on the stack|
|Pop   | None       |None|Removes the value off the top of the stack and discards it|
|Dup   | None   | TopValue|Duplicates the top value of the stack|
|Swap  | None   | SecondValue,FirstValue|Pops the top two values on the stack and swaps their order|
|StoreLocal| u8| None| Pop's the top value off the stack and stores it in local storage specified by a u8|
|LoadLocal| u8| StoredValue| Pushes a value onto the stack from local storeage specified by a u8|
|Add| None |Sum|Pops the two top values off the stack and adds them together. They must be the same primitive type|
|Subtract| None |Difference|Pops the two top values off the stack and subtracts them. They must be the same primitive type|
|Multiply| None |Product|Pops the two top values off the stack and multiply them together. They must be the same primitive type|
|Divide| None |Quotient|Pops the two top values off the stack and divides them. They must be the same primitive type|
|Modulo| None |Modulus|Pops the two top values off the stack and fetches the modulos of the two numbers. They must be the same primitive type|
|Negate| None | None |Negates the numerical value of the top of the stack. This only works on pimitive numbers. An error will occur if the negated value cannot be stored in its signed counterpart|
|AND| None | Product |Performs bitwise and between two primitive numbers. They must be the same primitive type|
|OR| None | Sum |Performs bitwise or between two primitive numbers. They must be the same primitive type|
|XOR| None | Sum |Performs bitwise xor between two primitive numbers. They must be the same primitive type|
|NOT| None | Inverse | Performs bitwise not on a primitive number|
|ShiftLeft| None | ShiftedValue | Performs a logical shift on all unsigned numbers and arithmetic shift on signed numbers. This needs a u32 from the stack to shift that amount|
|ShiftRight| None | ShiftedValue | Performs a logical shift on all unsigned numbers and arithmetic shift on signed numbers. This needs a u32 from the stack to shift that amount|
|Equal| None | i8: 0 if equal, anything else for not equal| Performs a comparison between two primitive types which must be the same type|
|Greater| None | i8: 1 if greather than, 0 for not greater than| Performs a comparison between two primitive types which must be the same type|
|Less| None | i8: -1 if less than, 0 for not less than| Performs a comparison between two primitive types which must be the same type|
|Convert| u8 | ConvertedValue | Converts the top value on the stack to a type specified by the u8,This is the same as doing `as` in Rust|
