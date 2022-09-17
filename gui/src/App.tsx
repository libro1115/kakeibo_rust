import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
          <div>
            <input 
            type="radio"
            id="income"
            name="usage"
            value={"income"}
            />
            <label>収入</label>
          </div>
          <div>
            <input 
            type="radio"
            id="spending"
            name="usage"
            value={"spending"}
            checked
            />
            <label>支出</label>
          </div><input id = "execute"
          placeholder="金額"
          maxLength={15} 
          />
          <div className="badget">
            <select name="badget">
              <option value={"week"}>週予算</option>
              <option value={"month"}>月予算</option>
              <option value={"two_month"}>2か月予算</option>
              <option value={"three_month"}>3か月予算</option>
              <option value={"four_month"}>4か月予算</option>
              <option value={"harf_annual"}>半年予算</option>
              <option value={"annual"}>年予算</option>
            </select>
            <p id="badget">残予算額：{}</p>
          </div>
        </div>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
