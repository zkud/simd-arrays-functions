const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'simd-arrays-functions' means native addon name is `simd-arrays-functions`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `simd-arrays-functions.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@simd-arrays-functions/simd-arrays-functions-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'simd-arrays-functions')
