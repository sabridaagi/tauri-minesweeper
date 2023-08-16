import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { Empty, Bomb, Hidden, Numbered } from './Cell';
import { splitArray } from '../utils/Array';

interface IProps {
  width: number;
  height: number;
  bombs: number;
}

function BoardTable(props: IProps) {
  const [board, setBoard] = useState<number[][]>([]);

  //Board generation at load
  useEffect(() => {
    invoke("generate_board", {
      width: props.width,
      height: props.height,
      numberBombs: props.bombs }).then(res => {
      let resBoard = JSON.parse(res as string);
      setBoard(splitArray(resBoard.cells, resBoard.width, resBoard.height));
    });
  }, []);

  //Right click
  useEffect(() => {
    const handleContextMenu = (e: any) => {
      e.preventDefault()
    }
    document.addEventListener("contextmenu", handleContextMenu)
    return () => {
      document.removeEventListener("contextmenu", handleContextMenu)
    }
  }, []);

  let handleBoxClick = (x: number, y: number) => {
    invoke("cell_clicked", { x: x, y: y }).then(res => {
      let resBoard = JSON.parse(res as string);
      setBoard(splitArray(resBoard.cells, resBoard.width, resBoard.height));
      console.table(board)
    });
  }

  let handleBoxRightClick = (x: number, y: number) => {
    invoke("cell_right_clicked", { x: x, y: y });
  }

  let handleBoxComponent = (x: number, y: number, value: number) => {
    switch(board[x][y]) {
      case 0: return <Empty />        // Empty
      case 9: return <Bomb />         // Bomb
      case 10: return <Hidden />      // Unopened
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
                onClick={() => handleBoxClick(i,j)}
                onContextMenu={() => handleBoxRightClick(i,j)}
              >
                
                {handleBoxComponent(i,j, board[i][j])}
              </td>
            )}
          </tr>
        )}
      </tbody>
    </table>
  )
}

export default BoardTable