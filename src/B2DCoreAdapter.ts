
function setupMacroquad() {
  const glScript = document.createElement('script');
  // TODO: Lock to specific version, not @latest
  glScript.src = 'https://not-fl3.github.io/miniquad-samples/gl.js';
  glScript.onload = () => {
    console.log('Loaded!');
    (window as any).load('b2d-core.wasm');
  }
  document.body.appendChild(glScript);
}

export async function testWasm() {
  const wasm = await import("b2d-core");
  wasm.greet();

  const canvas = document.createElement('canvas');
  document.body.appendChild(canvas);
  canvas.width = 500;
  canvas.height = 500;
  wasm.draw(canvas.getContext('2d')!, canvas.width, canvas.height, -Math.random(), Math.random());
  
}
