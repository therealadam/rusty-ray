# Rusty Ray

Wherein I get dirty with Rust and Jamis Buck's [The Ray Tracer Challenge](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge). I am probaby writing some very substandard Rust here, so bear with me.

## Caveat organizer

Cargo implies a `prelude` to every crate. Don't be like me and try to create your own bespoke little language of global types and factory functions in prelude! Or, if you do, know more about how Rust modules and Cargo interact.

You can get a long way with one file in Rust. Multiple modules, tests, a main function, all in one place. Again: know how `use`, `mod`, and filesystems path. In particular, this changed a bit between Rust 2015 and Rust 2018, so make sure you're understanding the right version.

## If you're coming from Ruby

The most jarring thing about Rust (or C, C++, anything you may have heard as a "systems" language) is that _a lot_ of details are handled/managed/abstracted by Ruby and now they're all in your face. For example: specifying how memory is allocated or lifetime-managed, specifying the exact shape of data (i.e. using structs instead of hashes or semi-open-ended classes), handling IO errors, specifying whether a variable should be referenced or copied. Languages that don't manage things for you like Ruby does can feel very unproductive at first. Stick with it!

Further, if you're coming from Rails, expect fewer decisions to be made for you. Rust makes more choices for you than e.g. React or C do, but not nearly as many as Rails.