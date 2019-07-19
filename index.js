const wasm = import('./pkg/stats_rs');

wasm.then(({ Stats }) => {
  const stats = Stats.init();

  stats.attach(document.body);

  function render() {
    stats.update();
    requestAnimationFrame(render);
  }

  render();
})
