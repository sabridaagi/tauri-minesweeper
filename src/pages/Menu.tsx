import "./Menu.css";

interface IProps {
  changeIngame(): void;
}

function Menu(props: IProps) {
  return (
    <div className="menu">
      <h1>Welcome to minesweeper game</h1>
      <button onClick={() => props.changeIngame()}>New game</button>
    </div>
  )
}

export default Menu;