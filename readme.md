# Causal Aquarium

## What's New

This project introduces an “aquarium” visualization that represents the magnitude of variables, providing a more intuitive interface for exploring causal relationships.

The cognitive task can be described as follows:  
> “We need to study the food chain relationships among these three types of fish (represented by three different shapes in this demo) and identify which type of fish affects the population of any other type — whether by increasing or decreasing it.”

By modifying the task description, adding more vivid aquarium details, and changing interaction modes, we can systematically explore how the intuitiveness of such a cognitive task — in one particular sense of *ecological validity* — influences cognitive strategies.

## Why Rust?

1. **High performance** — Rust is extremely fast, comparable to C++, which is important for running real-time simulations efficiently.  
2. **Strong type system** — Rust's type system catches many errors at compile time, improving the reliability of experimental code.  
3. **Pattern matching** — Rust's powerful pattern matching simplifies handling of experimental states and events.  
4. **Memory safety** — Rust ensures memory safety without a garbage collector, reducing runtime errors and simplifying maintenance.  
5. **Growing ecosystem** — Rust provides tools and libraries across many domains, enabling flexible extensions (though its ecosystem is still maturing compared to Python).

## What's Next

- [ ] Add a recording system to log experimental data  
- [ ] Rearrange the UI  
- [ ] Enrich the details of the aquarium and fish  
- [ ] Add a historical variable chart to display changes in each variable over the past 10 seconds  

## How to Deploy

1. Install [rustup](https://rustup.rs/)
2. run `rustup target add wasm32-unknown-unknown`  (see [yew](https://yew.rs/docs/getting-started/introduction) for futher information)
3. run `cargo install --locked trunk` (see [yew](https://yew.rs/docs/getting-started/introduction) for futher information)
4. `clone` this responsitory and navigate to the project directory
5. run the app by `trunk serve --open`
