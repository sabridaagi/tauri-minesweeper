import React from "react"

let Empty = () => {
  return (
    <div style={{ backgroundColor: "rgb(207, 204, 204)" }} />
  )
}

let Numbered = () => {
  return (
    <div className="empty" />
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