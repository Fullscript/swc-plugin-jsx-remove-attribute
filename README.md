# swc-plugin-jsx-remove-attribute

swc-plugin-jsx-remove-attribute is a plugin for [swc](https://swc.rs/) used to remove a specified set of attributes from JSX elements.

**NOTE:** we don't have the capacity to maintain this all the time, bug support and updates are not guaranteed.

## Prerequisites

If you don't already have **swc** setup, you can follow their [getting started guide](https://swc.rs/docs/getting-started).

## Installation
Add **swc-plugin-jsx-remove-attribute** to your dependencies like so:

Yarn v1:
- `yarn add https://github.com/Fullscript/swc-plugin-jsx-remove-attribute.git#1.0.0`

Yarn v2 (and onwards):
- `yarn add @fullscript/swc-plugin-jsx-remove-attribute@https://github.com/Fullscript/swc-plugin-jsx-remove-attribute.git#1.0.0`

NPM:
- `npm install https://github.com/Fullscript/swc-plugin-jsx-remove-attribute.git#1.0.0`

## Configuration

Wherever your SWC configuration is located, add the following:
```js
{
  jsc: {
    ...
    experimental: {
      plugins: [
        [
            "@fullscript/swc-plugin-jsx-remove-attribute",
            {
              // You can add as many attribute names in here as you want
              attributesToRemove: ["data-testid"],
            },
          ],
      ]
    }
  }
}
```

## Contributing

Bug reports and pull requests are welcome :)

### Building for release

1. Run: `cargo build-wasi --release`
2. Copy the resulting wasm file to the root of the project:
`cp ./target/wasm32-wasi/release/swc_plugin_jsx_remove_attribute.wasm .`
3. Commit and push!


