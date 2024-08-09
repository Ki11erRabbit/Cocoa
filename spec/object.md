# Object Structure
An object is a fat pointer that contains:
* A pointer to a list of attached traits
* A pointer to the fields of the object

The pointer to attached traits is so that we know how to treat an object as that trait. 

The pointer of fields of the object is where the data of the object is stored


## Strutures of sub parts

#### List of Attached Traits
The reason why they are attached is because Traits can be defined anywhere and implementations could be found nearly anywhere, they have to be attached at call sites. Luckily, once attached it doesn't need to be attached again since all objects share the same list


#### Field Pointer
This is just a list of members which can be primative types or a pointer to another object.
