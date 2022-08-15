import React, { HTMLAttributes } from 'react'
import style from './gamecard.module.css'
import { CameraOff } from 'react-feather'
import classNames from 'classnames'

type GameCardProps = {
  game: Game
} & HTMLAttributes<HTMLDivElement>

function GameCard(props: GameCardProps): JSX.Element {
  return (
    <div className={classNames(style.game_card, props.className)}>
      <div className={style.preview}>
        <CameraOff className={style.no_image} />
      </div>
      <div className={style.info}>
        <h1 className={style.title}>{props.game.name}</h1>
        <p className={style.author}>{props.game.author}</p>
      </div>
    </div>
  )
}

export default GameCard
