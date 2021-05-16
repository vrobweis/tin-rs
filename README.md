# tin-rs
A Rust language adaptation of Loren Olson's Tin Framework, a framework in the Swift language for general media arts programming. This itself inherits from examples in The Nature of Code by Daniel Shiffman.

This framework can be used to make simple programmatic drawings or complex particle systems in 2D space by using and extending functionality that is easily invoked with little boilerplate. 

# Objectives
- **Simplicity in use.** 
  - The original framework was used for students in an educational environment. This framework should be designed with the goal of being easy to learn, but robust enough to support complicated drawings.
- **Extensible in features.** 
  - The framework supports the drawing of simple geometric shapes, but as it can be used for complex drawings, it should permit the construction of more complicated graphics systems.
- **Conforms to Rust idioms.** 
  - The original Tin framework was designed in Swift, and made use of language features that fit the environment like inheritance and nullable fields for dependency injection. This adaptation should conform to those design patterns, but within the bounds of Rust patterns, replicating the benefits of the Swift design without the specific Swift features. This means that this framework will not match Swift behind the scenes, but should match it when invoked and used.
