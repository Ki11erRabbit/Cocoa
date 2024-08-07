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
* 
