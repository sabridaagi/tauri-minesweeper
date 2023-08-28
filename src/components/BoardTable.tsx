import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { Empty, Bomb, Hidden, Numbered } from './Cell';
import { splitArray, contains } from '../utils/Array';

interface IProps {
  width: number;
  height: number;
  bombs: number;
  onGameOver(): void;
}

let BoardTable = (props: IProps) => {
  const [board, setBoard] = useState<number[][]>([]);
  const [gameOver, setGameOver] = useState<boolean>(false);

  // Board generation at load
  useEffect(() => {
    invoke("generate_board", {
      width: props.width,
      height: props.height,
      numberBombs: props.bombs }).then(res => {
      let resBoard = JSON.parse(res as string);
      setBoard(splitArray(resBoard.cells, resBoard.width, resBoard.height));
    });
  }, []);

  // disable right click through the board
  useEffect(() => {
    const handleContextMenu = (e: any) => {
      e.preventDefault()
    }
    document.addEventListener("contextmenu", handleContextMenu)
    return () => {
      document.removeEventListener("contextmenu", handleContextMenu)
    }
  }, []);

  // update when gameOver
  useEffect(() => {
    if(gameOver) {
      props.onGameOver();
    }
  }, [gameOver]);

  let handleBoxClick = (x: number, y: number) => {
    if(!gameOver)
      invoke("cell_clicked", { x: x, y: y }).then(res => {
        let board = JSON.parse(res as string);
        let splitBoard = splitArray(board.cells, board.width, board.height);

        // look for potential loss
        if(contains(splitBoard, 9)) {
          setGameOver(true);
        }

        setBoard(splitBoard);
      });
  }

  let handleBoxRightClick = (x: number, y: number) => {
    invoke("cell_right_clicked", { x: x, y: y });
  }

  let Cell = (x: number, y: number, value: number) => {
    switch(board[x][y]) {
      case 0: return <Empty />        // Empty
      case 9: return <Bomb />         // Bomb
      case 10: return <Hidden />      // Hidden
      default: return <Numbered value={value} />    // Number
    }
  }

  return (
    <table>
      <tbody>
        {board.map((rows, i) =>
          <tr key={i}>
            {rows.map((_, j) =>
              <td
                key={j}
                onClick={() => handleBoxClick(j, i)}
                onContextMenu={() => handleBoxRightClick(j, i)}
              >
                {Cell(i,j, board[i][j])}
              </td>
            )}
          </tr>
        )}
      </tbody>
    </table>
  )
}

export default BoardTable;