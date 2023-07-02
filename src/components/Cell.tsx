import React from "react"

let Empty = () => {
  return (
    <div style={{ backgroundColor: "rgb(207, 204, 204)" }} />
  )
}

let Numbered = (props: { value: number }) => {
  return (
    <div style={{ backgroundColor: "rgb(207, 204, 204)" }}>
      {props.value}
    </div>
  )
}
let Bomb = () => {
  return (
    <div className="empty" />
  )
}

let Hidden = () => {
  return (
    <div style={{ backgroundColor: "rgb(79, 79, 79)" }} />
  )
}
export { Empty, Hidden, Numbered, Bomb }