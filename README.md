# fastcdc-wasm

Another attempt at creating a wrapper for fastcdc in node.js.  This time using wasmbindgen instead of neon.

Has a few sharp edges, but mostly works.

## install

```
npm install fastcdc-wasm
```

## API

### `require('fastcdc-wasm')(bytes, minChunkSize, avgChunkSize, maxChunkSize)`

* `bytes` some `Uint8Array` of bytes
* `minChunkSize` the smallest size chunk
* `avgChunkSize` the average chunk size
* `maxChunkSize` maximum chunk size

**Returns** A `Uint32Array` of chunks

## License
(c) 2021 Mikola Lysenko. MIT License