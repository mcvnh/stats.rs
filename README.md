Webassembly Performance Monitor

Stats is a webassembly version of [stats.js](https://github.com/mrdoob/stats.js) written in Rust.

This crate provides a simple info box that will help you monitor your code performance,
and can be used either Javascript code or in Rust code.

* **FPS** Frames rendered in the last second. The higher the number the better.
* **MS** Milliseconds needed to render a frame. The lower the number the better.

#### Examples

In Javascript
```js
const wasm = import('./pkg/stats_rs');

wasm.then(Stats => {
  const stats = Stats.init();
  stats.attach(document.body);

  function render() {
    stats.update();
    requestAnimationFrame(render);
  }

  render();
});
```

### Rust
```
// TODO
```
