import React from "react";
import { useState } from "react";
import Game from "./pages/Game";
import Menu from "./pages/Menu";
import "./styles/App.css";

const App = () => {
  const [ingame, setIngame] = useState(false);

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
