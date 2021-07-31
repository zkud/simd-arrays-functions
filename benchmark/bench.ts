import b from 'benny'

import { add, mul } from '../index'

function jsSum(a: Float64Array, b: Float64Array, c: Float64Array) {
  for (let index = 0; index < c.length; index++) {
    c[index] = a[index] + b[index]
  }
}

function jsMul(a: Float64Array, b: Float64Array, c: Float64Array) {
  for (let index = 0; index < c.length; index++) {
    c[index] = a[index] * b[index]
  }
}

const first = new Float64Array(100000)
const second = new Float64Array(100000)
const res = new Float64Array(100000)

async function run() {
  await b.suite(
    'Sum',

    b.add('Rust sum of arrays', () => {
      add(first, second, res)
    }),

    b.add('JavaScript sum of arrays', () => {
      jsSum(first, second, res)
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'Mul',

    b.add('Rust mul of arrays', () => {
      mul(first, second, res)
    }),

    b.add('JavaScript mul of arrays', () => {
      jsMul(first, second, res)
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'Complex Expression',

    b.add('Rust complex exp of arrays', () => {
      add(first, res, res)
      mul(first, res, res)
      mul(second, res, res)
    }),

    b.add('JavaScript mul of arrays', () => {
      jsSum(first, res, res)
      jsMul(first, res, res)
      jsMul(second, res, res)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
