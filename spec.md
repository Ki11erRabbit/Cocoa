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
There needs to be context struct that holds a reference to all local objects to a thread.
This is one layer of indirection because all references instead point to a global object table that holds all objects.
These objects by default are not protected by mutexes in order to preserve speed in low threaded applications.
Attempting to access an object that isn't protected in another thread than the creating thread should cause the equivalent of a page fault which should cause the object to be protected to become referencable in the other thread.


