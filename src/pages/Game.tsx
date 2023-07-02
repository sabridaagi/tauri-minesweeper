import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import BoardTable from "../components/BoardTable";
import "../styles/Game.css";
import { splitArray } from "../utils/Array";

interface IProps {
  changeIngame(): void;
}

const Game = (props: IProps) => {

  return (
    <React.Fragment>
      <div className="game">
        <h1>Game</h1>
        <button onClick={() => props.changeIngame()}>Leave game</button>
        <BoardTable width={35} height={20} bombs={1} />
      </div>
    </React.Fragment>
  );
}

export default Game