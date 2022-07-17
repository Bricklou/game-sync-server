import React from 'react'
import style from './gamecard.module.css'
import { CameraOff } from 'react-feather'

interface GameCardProps {
  game: Game
}

function GameCard(props: GameCardProps): JSX.Element {
  return (
    <div className={style.game_card}>
      <div className={style.preview}>
        <CameraOff className={style.no_image} />
      </div>
      <div className={style.info}>
        <h1 className={style.title}>{props.game.name}</h1>
        <p>{props.game.author}</p>
      </div>
    </div>
  )
}

export default GameCard
