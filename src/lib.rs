#![deny(clippy::all)]
mod ops;
mod util;

#[macro_use]
extern crate napi_derive;

use napi::{JsObject, Result};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("add", ops::add::add)?;
  exports.create_named_method("mul", ops::mul::mul)?;
  Ok(())
}
