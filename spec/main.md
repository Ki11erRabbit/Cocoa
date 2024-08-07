# High Level Overview
Cocoa, a garbage collected systems language with inspirations from Java, C#, Haskell, and Smalltalk

## Organization
Everything split into Cocoa `.cocoa` files which contain a package declaration, import declarations, and then function, class, and interface implementations in some order.

## Goals
* Generics can accept any type and not just objects.
* Wrappers shouldn't hold all of the methods that primitives need
  * Primitives should have a method-like interface
* Operator overloading should be done via special interfaces
* Higher kinded types so that interfaces are more like typeclasses to allow things like monads. This might be done through some kind of implicit parameter or some other compiler or runtime magic.
* Lambda expressions/anonymous objects with non-const (non-final) captures
* A configurable GC that is generational with reuse.


## Features
* Java-like single inheritance
* Interfaces
  * Can act as typeclasses
  * Implementation is decoupled from class instantiation but must follow orphan rule
  * Can be implemented on any type
* Signed and Unsigned integers
* Generics are done through reification to allow primitives into generics
  * Generics can also be bounded by interfaces and superclasses
