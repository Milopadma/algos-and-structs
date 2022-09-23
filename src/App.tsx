import reactLogo from "./assets/react.svg";
import { validAnagram } from "./scripts/section5";
import { countdown } from "./scripts/section7";
import "./App.css";

function App() {
  return (
    <div className="App">
      <div>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Console</h1>
      <div className="card">{validAnagram("anagram", "nagaram")}</div>
      <div className="card">{countdown(5)}</div>
    </div>
  );
}

export default App;
