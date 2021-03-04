export async function testWasm() {
  const wasm = await import("b2d-core");
  wasm.greet();
  
}
