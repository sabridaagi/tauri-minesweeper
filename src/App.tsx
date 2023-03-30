import React from "react";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Game from "./pages/Game";
import Menu from "./pages/Menu";

const App = () => {

  const [ingame, setIngame] = useState(false);

  //KEEPING FOR UNDERSTANDING
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <React.Fragment>
    {ingame ? 
        <Game changeIngame={() => setIngame(!ingame)} />
      :
        <Menu changeIngame={() => setIngame(!ingame)} />
    }
    </React.Fragment>
  );
}

export default App;
