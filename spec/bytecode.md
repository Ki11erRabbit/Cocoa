# Bytecode Definition

### Type conversion
|Alias | Type|
|:----:|:---:|
|PoolPointer| u64 |
| PoolValue| u8,i8,u16,i16,u32,i32,u64,i64 |
|SymbolPointer| u64|

### Bytecode Definition
| Operation | Operands   |  Output | Description |
|:---------:|:----------:|:-------:|:------------|
|LoadConstant|PoolPointer|PoolValue|Takes a value from the constant pool and puts it on the stack|
|StoreConstant|PoolPointer| None |Pops a value off of the operand stack and puts it into the constant pool.|
|Pop   | None       |None|Removes the value off the top of the stack and discards it|
|Dup   | None   | TopValue|Duplicates the top value of the stack|
|Swap  | None   | SecondValue,FirstValue|Pops the top two values on the stack and swaps their order|
|StoreLocal| u8| None| Pop's the top value off the stack and stores it in local storage specified by a u8|
|LoadLocal| u8| StoredValue| Pushes a value onto the stack from local storeage specified by a u8|
|StoreArgument| None | None | Pops the top value of the stack and stores it into the Argument List|
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
|BinaryConvert| u8 | ConvertedValue | Converts the top value on the stack to another type via its binary equivalence|
|Goto| i64 | None | offsets the PC by the amount specified by i64|
|Jump| None | None | Pops a i64 from the top of the stack and uses it to offset the PC|
|If| i64 | None | Offsets the PC by the amount specified by i64 if the top value on the stack is 0|
|IfNot| i64 | None | Offsets the PC by the amount specifed by i64 if the top value on the stack is not 0|
|IfGreater| i64 | None | Offsets the PC by the amount specifed by i64 if the top value on the stack is 1 or greater|
|IfGreaterEqual| i64 | None | Offsets the PC by the amount specifed by i64 if the top value on the stack is 0 or greater|
|IfLess| i64 | None | Offsets the PC by the amount specifed by i64 if the top value on the stack is -1 or less|
|IfLessEqual| i64 | None | Offsets the PC by the amount specifed by i64 if the top value on the stack is 0 or less|
|IfNull| i64 | None | Offsets the PC the amount specified by the i64 if the top value on the stack is a reference that is null (0)|
|IfNotNull| i64 | None | Offsets the PC the amount specified by the i64 if the top value on the stack is a reference that is not null (not 0)|
|InvokeFunction| SymbolPointer | None or Value | Invokes a new function and loads the Argument List into the next frame's local variables, pushes the PC onto the current operand stack, and can return a value or nothing|
|InvokeFunctionTail| SymbolPointer | None | Invokes a new function and loads the Argument List into the current frames's local variables, resets the PC to zero. This does not add any new frames to the runtime stack but allows for stacktraces of tail recursive functions|
|InvokeTrait| SymbolPointer,u64 | None or Value | Invokes a new function and loads the Argument List into the next frames's local variables, pushes the PC onto the current operand stack, and can return a value or nothing. The function comes from the trait list of the object and is found via the symbol pointer and the u64 is the index into the trait vtable|
|InvokeTraitTail| SymbolPointer | None | Invokes a new function and loads the Argument List into the current frames's local variables, resets the PC to zero. This does not add any new frames to the runtime stack but allows for stacktraces of tail recursive functions. The function comes from the trait list of the object and is found via the symbol pointer and the u64 is the index into the trait vtable|
|Return| None | None | Pops the top value off of the operand stack and puts it onto the top of the previous frame's operand stack. Pops the current stack frame off, swaps the top two values on the prevous frames's operand stack and pop's the previous PC off of the operand stack|
|ReturnUnit| None | None | Pops the current stack frame off, and pops the previous PC off of the operand stack and loads it.|
|CreateObject| SymbolPointer | Reference | Uses the symbol pointer and the Argument List to construct a new object based off of the symbol|
|CreateEnum| SymbolPointer | Reference | Uses the symbol pointer and the Argument List to construct a new enum object based off of the symbol|
|IsA| PoolPointer| 0 if true, any other value if false| This checks if the top value on the stack is what the symbol pointed by the PoolPointer says it is| 
|GetField| u64,u8 | Value | Offsets an object's data by the u64 and uses the u8 to figure out the type of the field|
|SetField| u64,u8 | None | Offsets an object's data by the u64 and uses the u8 to set the correct type in the object|
|CreateArray| u8 | Array | Creates an array by popping a u64 off of the stack for the size and uses the u8 for the type for the array|
|ArrayGet| u8 | Value | gets a value from an array on the stack by popping an u64 off of the stack and uses the u8 for the type|
|ArraySet| u8 | None | pops a value off of the stack, an index (u64) off of the stack, and uses the u8 for the type|
|Breakpoint| None | None | pauses execution until notified to continue executing. Used for debugging |
|Nop| None | None | Does nothing but increment the PC|
