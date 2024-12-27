
# The `wasm-bindgen` Guide

# Importing non-browser JS

View full source code or view the compiled example online

The #[wasm\_bindgen] attribute can be used on extern "C" { .. } blocks to import
functionality from JS. This is how the js-sys and the web-sys crates are
built, but you can also use it in your own crate!

For example if you're working with this JS file:

```
// defined-in-js.js
export function name() {
    return 'Rust';
}

export class MyClass {
    constructor() {
        this.\_number = 42;
    }

    get number() {
        return this.\_number;
    }

    set number(n) {
        return this.\_number = n;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}
```

you can use it in Rust with:

```
# #![allow(unused\_variables)]
#fn main() {
use wasm\_bindgen::prelude::*;

#[wasm\_bindgen(module = "/defined-in-js.js")]
extern "C" {
    fn name() -> String;

    type MyClass;

    #[wasm\_bindgen(constructor)]
    fn new() -> MyClass;

    #[wasm\_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;
    #[wasm\_bindgen(method, setter)]
    fn set\_number(this: &MyClass, number: u32) -> MyClass;
    #[wasm\_bindgen(method)]
    fn render(this: &MyClass) -> String;
}

// lifted from the `console\_log` example
#[wasm\_bindgen]
extern "C" {
    #[wasm\_bindgen(js\_namespace = console)]
    fn log(s: &str);
}

#[wasm\_bindgen(start)]
fn run() {
    log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"

    let x = MyClass::new();
    assert\_eq!(x.number(), 42);
    x.set\_number(10);
    log(&x.render());
}

#}
```

You can also explore the full list of ways to configure imports