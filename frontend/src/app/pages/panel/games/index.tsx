import React, { useEffect, useState } from 'react'
import style from '@/styles/pages/games/games.module.css'
import Button from '@/components/Button'
import { FolderPlus } from 'react-feather'
import { Link, useNavigate } from 'react-router-dom'
import API from '@/utils/API'
import GameCard from '@/components/games/GameCard'
import Input from '@/components/Input'

function Games(): JSX.Element {
  const navigate = useNavigate()

  const [games, setGames] = useState<Game[]>([])
  const [page] = useState(1)
  const [search, setSearch] = useState('')

  useEffect(() => {
    const timeout = setTimeout(() => {
      API.get<Paginated<Game>>('/games', {
        params: {
          search,
          page
        }
      })
        .then((res) => {
          setGames(res.data.items)
        })
        .catch((err) => console.log(err))
    }, 200)

    return () => clearTimeout(timeout)
  }, [search])

  return (
    <div className={style.games}>
      <header>
        <h1>Registered games</h1>

        <Button classNames={style.add_btn} onClick={() => navigate('/games/add')}>
          <FolderPlus />
          Add new game
        </Button>
      </header>

      <main className={style.content}>
        <div className={style.search_bar}>
          <Input
            id="search"
            name="search"
            type="search"
            placeholder="Search a game..."
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <div className={style.games_list}>
          {games.map((game) => (
            <Link key={game.id} to={`/games/${game.id}`}>
              <GameCard game={game} />
            </Link>
          ))}
        </div>
      </main>
    </div>
  )
}

export default Games
