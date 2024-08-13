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
* Object Table
  * This structure holds all objects that are declared in the module
  * This structure also gets emptied only when needed
  * This structure can be larger than the amount of objects declared in source due to monomorphization
* Enum Table
  * This structure holds the definitions of enums
  * This structure also gets emptied but only when needed
  * These are tagged unions although since objects, each variant only takes up a pointer size
* Trait Table
  * Holds the trait definition
* Trait Impl Table
  * Holds the various implementations of traits following the orphan rule
* Location Table
  * Holds the line and column number of each bytecode instruction 

### Binary Format
#### Constant Pool
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

#### Function Table
The function table is as follows in binary:
```
FunctionTable {
 length: u64,
 table: [Function],
}
```
where a function is:
```
Function {
 name: u64,         // Symbol Index in constant pool
 symbolName: u64,   // If not monomorphized then this is the same as name
 typeInfo: u64,     // TypeInfo index in constant pool
 locationIndex: u64,// Index into location table
 flags: u8,
 codeLength: u64,
 bytecode: [u8],
}
```

#### Object Table
The object table is as follows in binary:
```
ObjectTable {
 length: u64,
 table: [Object],
}
```
where a `Object` is as follows:
```
Object {
 name: u64,            // Symbol index in constant pool
 symbolName: u64,      // If not monomorphized then this is the same as name
 flags: u8,
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
 flags: u8,
}
```
where `MethodInfo` is
```
MethodInfo {
 name: u64,
 symbolName: u64,
 typeInfo: u64,
 locationIndex: u64,
 flags: u8,
 codeLength: u64,
 code: [u8],
}
```
#### Enum Table
The enum table is as follows in binary:
```
EnumTable {
 length: u64,
 table: [Enum],
}
```
where Enum is:
```
Enum {
 name: u64,
 symbolName: u64,
 flags: u8,
 variantsCount: u8,
 variants: [EnumVariant],
 methodCount: u32,
 methods: [MethodInfo],
}
```
where EnumVariant is:
```
EnumVariant {
 fieldCount: u32,
 fields: [FieldInfo],
}
```
#### Trait Table
The trait table is as follows in binary:
```
TraitTable {
 length: u64,
 table: [Interface],
}
```
where Trait is defined as
```
Trait {
 name: u64,
 parentNameCount: u64,
 parentNames: [u64],
 genericParameterCount: u32,
 genericParameters: [GenericParameter],
 flags: u8,
 methodCount: u32,
 methods: [MethodInfo],
}
```
where GenericParameter is
```
GenericParameter {
 name: u64,
 bound: u64,
}
```
#### Trait Impl Table
The trait impl table is as follows in binary:
```
TraitImplTable {
 length: u64,
 table: [TraitImpl],
}
```
where TraitImpl is
```
TraitImpl {
 name: u64,
 symbolName: u64,
 methodCount: u32,
 methods: [MethodInfo],
}
```
#### Location Table
The interface table is as follows in binary:
```
LocationTable {
 length: u64,
 table: [Locations],
}
```
where Locations is
```
Locations {
 length: u64,         // The length should be the same as the bytecode it is representing
 locations: [(32,32)], 
}
```
