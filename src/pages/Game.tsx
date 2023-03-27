import React from "react";
import "./Game.css";

interface IProps {
  changeIngame(): void;
}

function Game(props: IProps) {
  return (
    <React.Fragment>
      <div>Game</div>
      <button onClick={() => props.changeIngame()}>Leave game</button>
    </React.Fragment>
  )
}

export default Game