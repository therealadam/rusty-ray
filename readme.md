Wherein I get dirty with Rust and Jamis Buck's [The Ray Tracer Challenge](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge). I am probaby writing some very substandard Rust here, so bear with me.

## Front matter

Why am I here, struggling through Rust, systems programming, and ray-tracing? Honestly, I ask myself that a lot when I'm struggling with something in Rust that is muscle-memory in Ruby!

First off, I wanted to do a thing in programming domain outside of my working-hours areas of expertise. Graphics programming and Rust are _exactly_ that. I heard lots of good stuff about Jamis Buck's first book on mazes, so using his test-driven approach with ray tracing, which I know practically zero things about, sounded like a good start. Further, having a capable author to hold my hand through the math bits is a thing I very much need.

Secondly, Rust seems like it has a lot of potential. It compliments Ruby in a lot of ways, has good stewardship and culture, and is well thought-out. In particular, I'm excited about its early leadership in compiling to Web Assembly, which seems like a particularly promising technology I want to get in on early. I hope to eventually port this stuff to WASM by working through [Programming Web Assembly with Rust](https://pragprog.com/book/khrust/programming-webassembly-with-rust).

Thirdly, I wanted a project I could stick with and work on in public. I would be slightly embarrased to put a web application up if it looked weird or didn't embody all the principles I think are important in building modern Rails apps. Not so with Rust and graphics programming!

So here I am, doing something very different, in public, and quasi-blogging without even using a blogging platform!

## Caveat organizer

Cargo implies a `prelude` to every crate. Don't do like me and try to create your own bespoke little language of global types and factory functions in prelude! Or, if you do, know more about how Rust modules and Cargo interact than I did.

You can get a long way with one file in Rust. Multiple modules, tests, a main function, all in one place. Again: know how `use`, `mod`, and filesystem paths. In particular, this changed a bit between Rust 2015 and Rust 2018, so make sure you're understanding the right version.

## If you're coming from Ruby

The most jarring thing about Rust (or C, C++, anything you may have heard as a "systems" language) is that _a lot_ of details are handled/managed/abstracted by Ruby and now they're all in your face. For example: specifying how memory is allocated or lifetime-managed, specifying the exact shape of data (i.e. using structs instead of hashes or semi-open-ended classes), handling IO errors, specifying whether a variable should be referenced or copied. Languages that don't manage things for you like Ruby does can feel very unproductive at first. Stick with it!

Further, if you're coming from Rails, expect fewer decisions to be made for you. Rust makes more choices for you than e.g. React or C do, but not nearly as many as Rails.
