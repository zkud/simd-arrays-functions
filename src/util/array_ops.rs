use napi::{CallContext, JsTypedArray, JsTypedArrayValue, Result};

#[inline]
pub fn extract_arrays_for_binary_op(
  ctx: &CallContext,
) -> Result<(JsTypedArrayValue, JsTypedArrayValue, JsTypedArrayValue)> {
  let (first_array, second_array, result_array) = extract_js_arrays(&ctx)?;

  validate_arrays(&ctx, &first_array, &second_array, &result_array)?;

  Ok((first_array, second_array, result_array))
}

fn extract_js_arrays(
  ctx: &CallContext,
) -> Result<(JsTypedArrayValue, JsTypedArrayValue, JsTypedArrayValue)> {
  Ok((
    ctx.get::<JsTypedArray>(0)?.into_value()?,
    ctx.get::<JsTypedArray>(1)?.into_value()?,
    ctx.get::<JsTypedArray>(2)?.into_value()?,
  ))
}

fn validate_arrays(
  ctx: &CallContext,
  first: &JsTypedArrayValue,
  second: &JsTypedArrayValue,
  result: &JsTypedArrayValue,
) -> Result<()> {
  if (first.length != second.length) || (second.length != result.length) {
    return ctx.env.throw_error("length of arrays does not match", None);
  }

  if (first.typedarray_type != second.typedarray_type)
    || (second.typedarray_type != result.typedarray_type)
  {
    return ctx.env.throw_error("type of arrays does not match", None);
  }

  Ok(())
}
