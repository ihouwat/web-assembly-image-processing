const path = require('path');
const HTMLWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: './public/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  plugins: [
    new HTMLWebpackPlugin({
      template: './public/index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.') // searches for cargo.toml
    })
  ],
  experiments: {
    asyncWebAssembly: true // enable support for WA, since it is still an experimental feature
  }
}