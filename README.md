[![build](https://github.com/vrobweis/tin-rs/actions/workflows/build.yml/badge.svg)](https://github.com/vrobweis/tin-rs/actions/workflows/build.yml)
# tin-rs
A Rust language adaptation of [Loren Olson's Tin Framework](https://lnolson.github.io/Tin/), a framework in the Swift language for general media arts programming. This itself inherits from examples in The Nature of Code by Daniel Shiffman.

This framework can be used to make simple programmatic drawings or complex particle systems in 2D space by using and extending functionality that is easily invoked with little boilerplate. 

# Objectives
- **Simplicity in use.** 
  - The original framework was used for students in an educational environment. This framework should be designed with the goal of being easy to learn, but robust enough to support complicated drawings.
- **Extensible in features.** 
  - The framework supports the drawing of simple geometric shapes, but as it can be used for complex drawings, it should permit the construction of more complicated graphics systems.
- **Conforms to Rust idioms.** 
  - The original Tin framework was designed in Swift, and made use of language features that fit the environment like inheritance and nullable fields for dependency injection. This adaptation should conform to those design patterns, but within the bounds of Rust patterns, replicating the benefits of the Swift design without the specific Swift features. This means that this framework will not match Swift behind the scenes, but should match it when invoked and used.


## Considerations 
Tin is not a high-performance framework. The original Swift version is effectively a simplified API wrapper that maps relatively easily to CoreGraphics calls, and fits neatly into the MVC paradigm found in a lot of the Mac development ecosystem. It uses global state to allow all the functions used to be trivially invoked. 

None of this is usually applicable to Rust and its environment. Stack allocation is the default. Many concepts, idioms, the entry point, and entire programming concepts used in the original framework are different in Rust. Global state is discouraged, and most graphics wrappers in Rust instead provide similar simplicity through the builder pattern creating a struct that then has methods to delegate draw calls. 

But this framework maintains the original simplicity of the Swift version with an implementation that most performance sensitive Rustaceans would cringe at, by replicating the original's global state and interface,

- tin-rs uses a static reference to a lazily loaded struct called a TinContext that maintains all drawing state data that the Tin draw functions modify. 
- The TinContext is referred to throughout the framework using functions that abstract over a RwLock, that incurs additional overhead. 
  - This is required to ensure thread safety, even though Tin is not a multithreaded framework.
- Each draw call modified the Tin global state before then issuing calls to the backend. The backend implementation is stateful as well, so that abstract draw queues can be maintained before any graphics API calls are issued.
- The framework relies heavily on 64 bit floats for extra precision, given that some of the noise and random functions provided would otherwise offset user-given values and push the margins of error outward.
- The author of this adaptation is a fickle baby without a Computer Science background and likes to make Computer Science buffs cry. /s

If you are looking for a simple graphics framework to setup and play around with the barebones basics needed to get used to Rust, welcome to Tin. If you are looking for a performant graphics framework written in Rust that conforms to all best practices, runs at peak efficiency, and can serve as the base for complicated game engines, look almost anywhere else. I would recommend [Nannou](https://nannou.cc/) or [processing-rs](https://github.com/rennis250/processing-rs) for frameworks with simplicity as a similar goal, that rely less on global state and heap allocations (at first glance).

### Credit
Original Tin framework created by Loren Olson.
Copyrighted © 2017 ASU. All rights reserved.
