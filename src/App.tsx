import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

class MyOption {
  id: string = "";
  label: string = "";
}

function App() {
  const [options, setOptions] = useState<MyOption[]>([]);
  const [productName, setProductName] = useState<string>("");
  
  async function enum_hid() {
    setOptions(await invoke("enum_hid"));
  }

  async function sel_hid(id: string) {
    setProductName(await invoke("sel_hid", {path: id}));
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
          <button type="button" onClick={() => enum_hid()}>
            Enumerate HID
          </button>
          <select onChange={(e) => sel_hid(e.target.value)}>
            {options.map((opt, idx) => {
              return <option value={opt.id}>[{idx}] {opt.label}</option>
            })}
          </select>
        </div>
      </div>
      <p>🤛{productName}🤜</p>
    </div>
  );
}

export default App;
