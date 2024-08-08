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
The vtable holds a list of all methods that the object could call. This is where overloaded methods live
