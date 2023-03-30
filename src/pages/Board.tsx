import { useEffect } from 'react';

interface IProps {
  board: number[][]
}

function Board(props: IProps) {

  useEffect(() => {
    const handleContextMenu = (e: any) => {
      e.preventDefault()
    }
    document.addEventListener("contextmenu", handleContextMenu)
    return () => {
      document.removeEventListener("contextmenu", handleContextMenu)
    }
  }, []);

  let boxClicked = (x: number, y: number) => {
    console.log(x,y);
  }

  let boxRightClicked = (x: number, y: number) => {
    console.log("right click", x,y);
  }
  
  return (
    <table>
      <tbody>
        {props.board.map((line, i) => 
          <tr key={i}>
            {line.map((column, j) => 
              <td 
                key={j} 
                onClick={() => boxClicked(i,j)}
                onContextMenu={() => boxRightClicked(i,j)}>
                  H
                </td>
            )}
          </tr>
        )}
      </tbody>
    </table>
  )
}

export default Board