import React, { useEffect } from "react";
import Board from "./Board";
import "./Game.css";

interface IProps {
  changeIngame(): void;
}

const Game = (props: IProps) => {
  let table: number[][] = [];
  //Populate table 2d array

  return (
    <React.Fragment>
      <div className="game">
        <h1>Game</h1>
        <button onClick={() => props.changeIngame()}>Leave game</button>
        <Board board={table} />
        {/* Generate a basic board */}

      </div>
    </React.Fragment>
  )
}

export default Game