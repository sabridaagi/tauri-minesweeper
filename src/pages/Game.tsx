import React, { useState }  from "react";
import BoardTable from "../components/BoardTable";
import "../styles/Game.css";

interface IProps {
  changeIngame(): void;
}

const Game = (props: IProps) => {
  const [msg, setMsg] = useState<string>(":)");

  return (
    <React.Fragment>
      <div className="game">
        <h1>{msg}</h1>
        <button onClick={() => props.changeIngame()}>Leave game</button>
        <BoardTable
          width={15}
          height={20}
          bombs={30}
          onGameOver={() => setMsg(":(")}
        />
      </div>
    </React.Fragment>
  );
}

export default Game