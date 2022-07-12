import React from 'react'
import style from '@/styles/pages/games/games.module.css'
import Button from '@/components/Button'
import { FolderPlus } from 'react-feather'
import { useNavigate } from 'react-router-dom'

function Games(): JSX.Element {
  const navigate = useNavigate()
  return (
    <div className={style.games}>
      <header>
        <h1>Registered games</h1>

        <Button classNames={style.add_btn} onClick={() => navigate('/games/add')}>
          <FolderPlus />
          Add new game
        </Button>
      </header>
    </div>
  )
}

export default Games
