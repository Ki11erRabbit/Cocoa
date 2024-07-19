# Cocoa Programming Language

## Specification

### Class Header Object
```rust
struct ClassHeader {
	name: PoolIndex,
	parent: PoolIndex,
	class_flags: ClassFlags,
	constant_pool_size: usize,
	constant_pool: [PoolEntry],
	interface_size: usize,
	interface_table: [PoolIndex],
	field_count: usize,
	field_table: [FieldInfo],
	method_count: usize,
	method_table: [MethodInfo],
}
```

#### Class Flags
```
Public
Const
Interface
Abstract
Synthetic - Not found in source code
```
#### Pool Entry
```rust
enum PoolEntry {
	Method(MethodData),
	String(&str),
	ClassInfo(ClassInfo),
	InterfaceImpl(InterfaceImpl),
	TypeInfo(TypeInfo),
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
}
```
##### Method Data
```rust
enum MethodData {
	Native(NativeMethodIndex),
	Bytecode(Box<[Bytecode]>),
}
```
##### Class Info
```rust
struct ClassInfo {
	name: PoolIndex
}
```
##### Interface Impl
```rust
struct InterfaceImpl {
	name: PoolIndex
}
```
##### Type Info
```rust
enum TypeInfo {
	U8,
	U16,
	U32,
	U64,
	I8,
	I16,
	I32,
	I64,
	Float,
	Char,
	Object(PoolIndex),
	Method(Vec<TypeInfo>, Box<TypeInfo>),
}
```
#### Field
```rust
struct FieldInfo {
	name: PoolIndex,
	flags: AccessFlags,
	type_info: PoolIndex,
}
```
##### Access Flags
```
Public
Private
Protected
Static
Const
Synthetic - Not found in source code
```
#### Method Info
```rust
struct MethodInfo {
	flags: MethodFlags,
	name: PoolIndex,
	type_info: PoolIndex
	location: PoolIndex,
}
```
##### Method Flags
```
Public
Private
Protected
Abstract
Static
Const
VaArgs
```

### Object Table
Objects are stored in a table accessible from all threads. It is up to the programmer to ensure thread safety.
Class Headers are also stored here.

#### Structures

##### Object Header
```rust
struct ObjectHeader {
    mark: GcMark,
    size: usize,
    class_index: usize,
    ptr: Object,
}
```

##### Object Table
```rust
struct ObjectTable {
    offset: usize,
    table: RwLock<Vec<Option<ObjectHeader>>>
}
```



