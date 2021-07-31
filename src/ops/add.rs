use crate::util::array_ops::extract_arrays_for_binary_op;
use napi::{CallContext, JsUndefined, Result, TypedArrayType};
use std::ops::Add;

#[js_function(3)]
pub fn add(ctx: CallContext) -> Result<JsUndefined> {
  let (first_array, second_array, mut result_array) = extract_arrays_for_binary_op(&ctx)?;

  match first_array.typedarray_type {
    TypedArrayType::Int32 => {
      let first_array: &[i32] = first_array.as_ref();
      let second_array: &[i32] = second_array.as_ref();
      let result_array: &mut [i32] = result_array.as_mut();

      add_with_cpu(first_array, second_array, result_array);
    }
    TypedArrayType::Float32 => {
      let first_array: &[f32] = first_array.as_ref();
      let second_array: &[f32] = second_array.as_ref();
      let result_array: &mut [f32] = result_array.as_mut();

      add_with_cpu(first_array, second_array, result_array);
    }
    TypedArrayType::Float64 => {
      let first_array: &[f64] = first_array.as_ref();
      let second_array: &[f64] = second_array.as_ref();
      let result_array: &mut [f64] = result_array.as_mut();

      add_with_cpu(first_array, second_array, result_array);
    }
    _ => {
      return ctx
        .env
        .throw_error("Unsupported type", None)
        .and_then(|_val| ctx.env.get_undefined())
    }
  }

  ctx.env.get_undefined()
}

#[inline]
fn add_with_cpu<T: Add + Copy>(first_array: &[T], second_array: &[T], result_array: &mut [T::Output]) {
  for index in 0..result_array.len() {
    result_array[index] = first_array[index] + second_array[index];
  }
}

#[cfg(test)]
mod tests {
  use super::add_with_cpu;

  #[test]
  fn it_adds_slices_with_cpu() {
    let first_array: [u128; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let second_array: [u128; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result_array: [u128; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    add_with_cpu(&first_array, &second_array, &mut result_array);

    assert_eq!(result_array, [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
  }
}