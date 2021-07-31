# `@zkud/simd-arrays-functions`

[![CI](https://github.com/zkud/simd-arrays-functions/actions/workflows/CI.yaml/badge.svg)](https://github.com/zkud/simd-arrays-functions/actions/workflows/CI.yaml)

npm package for Node.js-like envs with simd functions for typed arrays implemented with Rust and Vulcano.

## Support matrix

### Operating Systems

|                  | node12 | node14 | node16 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

### Languages

Both TypeScript and JavaScript are supported.

## Installing @zkud/simd-arrays-functions

Installing SIMD requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support) and [vulkan API installed](https://www.vulkan.org/).

You can install the project with npm. In the project directory, run:

```sh
$ npm install @zkud/simd-arrays-functions
```

Or you can install the project with yarn. In the project directory, run:

```sh
$ yarn add @zkud/simd-arrays-functions
```

## Using @zkud/simd-arrays-functions

First we need to create typed arrays,
Regarding supported types please check [this documentation](123).
```
  const array1 = new Float64Array([1, 2, 3]);
  const array2 = new Float64Array([1, 2, 3]);
  const resultArray = new Float64Array(3);
```

Then we could use some operations, you could check the full list [here](123).
```
  const { add, mul } = require('@zkud/simd-arrays-functions');

  add(array1, array2, resultArray);
  mul(array1, resultArray, resultArray);
```

And then we need to extract result values, the **.values()** method of typed arrays could be used:
```
  for(const value of resultArray.values) {
    console.log(value);
  }
```
