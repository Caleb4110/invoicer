import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Tab from "./components/Tab/Tab";
import Clients from "./pages/Clients/Clients"
import { useState } from "react";

function App() {

  const [tabIndex, setTabIndex] = useState<number>(0);

  async function greet() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    //setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <div className="tab-container">
        <Tab index={0} isActive={tabIndex === 0} label="Clients" onClick={() => setTabIndex(0)} />
        <Tab index={1} isActive={tabIndex === 1} label="Invoices" onClick={() => setTabIndex(1)} />
      </div>
      <div className="content-container">
        {tabIndex === 0 ? <Clients /> : "HI"}
      </div>
    </main>
  );
}

export default App;
