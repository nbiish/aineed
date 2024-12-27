
# The `wasm-bindgen` Guide

# js-sys: WebAssembly in WebAssembly

View full source code or view the compiled example online

Using the js-sys crate we can get pretty meta and instantiate WebAssembly
modules from inside WebAssembly modules!

## src/lib.rs

```
# #![allow(unused\_variables)]
#fn main() {
use js\_sys::{Function, Object, Reflect, WebAssembly};
use wasm\_bindgen::prelude::*;
use wasm\_bindgen\_futures::{spawn\_local, JsFuture};

// lifted from the `console\_log` example
#[wasm\_bindgen]
extern "C" {
    #[wasm\_bindgen(js\_namespace = console)]
    fn log(a: &str);
}

macro\_rules! console\_log {
    ($($t:tt)*) => (log(&format\_args!($($t)*).to\_string()))
}

const WASM: &[u8] = include\_bytes!("add.wasm");

async fn run\_async() -> Result<(), JsValue> {
    console\_log!("instantiating a new Wasm module directly");

    let a = JsFuture::from(WebAssembly::instantiate\_buffer(WASM, &Object::new())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn\_into()?;

    let c = b.exports();

    let add = Reflect::get(c.as\_ref(), &"add".into())?
        .dyn\_into::<Function>()
        .expect("add export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    console\_log!("1 + 2 = {:?}", three);
    let mem = Reflect::get(c.as\_ref(), &"memory".into())?
        .dyn\_into::<WebAssembly::Memory>()
        .expect("memory export wasn't a `WebAssembly.Memory`");
    console\_log!("created module has {} pages of memory", mem.grow(0));
    console\_log!("giving the module 4 more pages of memory");
    mem.grow(4);
    console\_log!("now the module has {} pages of memory", mem.grow(0));

    Ok(())
}

#[wasm\_bindgen(start)]
fn run() {
    spawn\_local(async {
        run\_async().await.unwrap\_throw();
    });
}

#}
```