import React from 'react';
import logo from './logo.svg';
import './App.css';
import { testWasm } from './B2DCoreAdapter';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <button onClick={testWasm}>Test WASM</button>

        <canvas id="canvas" height="150" width="150"></canvas>
      </header>
    </div>
  );
}

export default App;
