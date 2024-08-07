# Module Files (`.mod`)
Module files are the compiled versions of `.cocoa` files. Module files contain all the information needed to compile new `.cocoa` files.
This allows for incremental compiling. 

## Structure
Module files contain the following data structures:
* Constant Pool
  * This structure holds all constants that are needed in a module file
  * It also holds the symbols and typing information of each item in a module.
* Function Table
  * This structure holds all toplevel function definitions and their respective bytecode.
  * This structure gets emptied out when it gets loaded into the VM
* Class Table
  * This structure holds all classes that are declared in the module
  * This structure also gets emptied only when needed
  * This structure can be larger than the amount of classes declared in source due to monomorphization
* Interface Table
  * Holds the interface definition
* Interface Impl Table
  * Holds the various implementations of interfaces following the orphan rule
 
### Constant Pool
The structure is as follows in binary
```
length: u64,
pool: [Entry],
```
Where an `Entry` is in the form of
```
tag: u8,
item: Item
```
where `Item` is one
```
u8 : 0
i8 : 1
u16 : 2
i16 : 3
u32 : 4
i32 : 5
u64 : 6
i64 : 7
f32 : 8
f64 : 9
char = {
 size : u8,
 data : [u8],
} : 10
string = {
 size : u64,
 data : [u8],
} : 11
Symbol = string : 12
TypeInfo : 13
```
where `TypeInfo` is one of
```
{
 tag: u8
}
{
 tag: u8
 count: u8,
 argType: [TypeInfo],
 returnType: TypeInfo,
}
```

### Function Table
The function table is as follows in binary:
```
FunctionTable {
 length: u64,
 table: [FunctionEntry],
}
```
where a function entry is:
```
FunctionEntry {
 name: u64,       // Symbol Index in constant pool
 symbolName: u64, // If not monomorphized then this is the same as name
 typeInfo: u64,   // TypeInfo index in constant pool
 codeLength: u64,
 bytecode: [u8],
}
```

### Class Table
The class table is as follows in binary:
```
ClassTable {
 length: u64,
 table: [Class],
}
```
where a `Class` is as follows:
```
Class {
 name: u64,            // Symbol index in constant pool
 symbolName: u64,      // If not monomorphized then this is the same as name
 parentName: u64,      // Symbol index in constant pool
 parentSymbolName: u64,// If not monomorphized then this is the same as parentName
 fieldCount: u32,      // Amount of fields
 fieldInfo: [FieldInfo],
 methodCount: u32,
 methods: [MethodInfo],
}
```
where `FieldInfo` is
```
FieldInfo {
 name: u64,
 typeInfo: u64,
}
```
where `MethodInfo` is
```
MethodInfo {
 name: u64,
 symbolName: u64,
 typeInfo: u64,
 codeLength: u64,
 code: [u8],
}
```
