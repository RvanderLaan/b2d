import React, { useEffect } from 'react';
import './App.css';
import { testWasm } from './B2DCoreAdapter';

function App() {
  useEffect(() => {
    testWasm().catch(console.error);
  })
  return (
    <div className="App">
      <header className="App-header">
        <canvas id="canvas"></canvas>
      </header>
    </div>
  );
}

export default App;
