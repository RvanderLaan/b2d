export async function testWasm() {
  const wasm = await import("b2d-core");
  wasm.greet();

  const canvas = document.createElement('canvas');
  document.body.appendChild(canvas);
  canvas.width = 500;
  canvas.height = 500;
  wasm.draw(canvas.getContext('2d')!, canvas.width, canvas.height, -Math.random(), Math.random());
  
}
