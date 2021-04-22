
export async function testWasm() {
  const wasm = await import("b2d-core");

  const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  // console.log(wasm, wasm.main);
  // wasm.draw(canvas.getContext('2d')!, canvas.width, canvas.height, -Math.random(), Math.random());
  wasm.main();
}
