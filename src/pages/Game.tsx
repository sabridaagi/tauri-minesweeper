import React  from "react";
import BoardTable from "../components/BoardTable";
import "../styles/Game.css";

interface IProps {
  changeIngame(): void;
}

const Game = (props: IProps) => {

  return (
    <React.Fragment>
      <div className="game">
        <h1>Game</h1>
        <button onClick={() => props.changeIngame()}>Leave game</button>
        <BoardTable width={15} height={20} bombs={99} />
      </div>
    </React.Fragment>
  );
}

export default Game