# rcanvas

## rlib

rlbi 使用的方法是拿到 DOM 中的 canvas 对象，然后在 rust 中执行 to_data_url，但是这个方式不太可行，耗时甚至比在 JS 中的耗时要长。

## rproject

这里是直接使用 rust 来绘制 canvas，然后直接导出 png.

这个方式是比较可行的，主要是要将所有的内容都使用 rust 中的 canvas 绘制出来。

## vue-test

该项目主要用来测试 wasm

## 其他方案

### webWorker

这个方式也不太可行，执行的时长比 JS 执行要长很多。

## Problems

* Uncaught (in promise) CompileError: WebAssembly.instantiate(): expected magic word 00 61 73 6d, found 0a 20 20 20 @+0
出现这个问题的原因是没有正确加载到对应的 wasm 资源。