# Object Structure
An object is a fat pointer that contains:
* A VTable Pointer
* A pointer to a parent object
* A pointer to a list of attached traits
* A pointer to the fields of the object

All object calls are done via dynamic dispatch. This allows for a standard interface.

The pointer to a parent object allows for inheritance and accessing of parent methods

The pointer to attached traits is so that we know how to treat an object as that trait. 

The pointer of fields of the object is where the data of the object is stored


## Strutures of sub parts

#### VTable
The vtable holds a list of all methods that the object could call. This is where overloaded methods live for the derived object.
The vtable holds a pointer into a method table. 
If the Function pointer is null (0), then the parent is used instead.
We also maintain a simmilar structure to the vtable that holds the symbols instead of functions. This is to allow for errors at runtime to be helpfull

#### Parent Pointer
This has the same structure as a normal object. If it is null (0) then we are the base object.

#### List of Attached Traits
The reason why they are attached is because Traits can be defined anywhere and implementations could be found nearly anywhere, they have to be attached at call sites. Luckily, once attached it doesn't need to be attached again.

Every derived class must implement the same traits that their parent implements.

#### Field Pointer
This is just a list of members which can be primative types or a pointer to another object.
