import React, { useEffect, useState } from 'react'
import style from '@/styles/pages/games/games.module.css'
import Button from '@/components/Button'
import { FolderPlus } from 'react-feather'
import { Link, useNavigate } from 'react-router-dom'
import API from '@/utils/API'
import GameCard from '@/components/games/GameCard'
import Input from '@/components/Input'
import Pagination from '@/components/Pagination'
import Select from '@/components/Select'

function Games(): JSX.Element {
  const navigate = useNavigate()

  const [games, setGames] = useState<Game[]>([])
  const [page, setPage] = useState(1)
  const [sort, setSort] = useState('name')
  const [totalPages, setTotalPages] = useState(1)
  const [search, setSearch] = useState('')

  useEffect(() => {
    const timeout = setTimeout(() => {
      API.get<Paginated<Game>>('/games', {
        params: {
          search,
          page,
          sort
        }
      })
        .then((res) => {
          setGames(res.data.items)
          setTotalPages(res.data.total_pages)
        })
        .catch((err) => console.log(err))
    }, 200)

    return () => clearTimeout(timeout)
  }, [page, search, sort])

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
          <Select
            name="sort_by"
            id="sort_by"
            selected="name"
            options={[
              { value: 'name', label: 'Name', selected: true },
              { value: 'author', label: 'Author' },
              { value: 'created_at', label: 'Created at' },
              { value: 'created_at_desc', label: 'Created at (desc)' },
              { value: 'updated_at', label: 'Updated at' },
              { value: 'updated_at_desc', label: 'Updated at (desc)' }
            ]}
            onChange={(e) => setSort(e.target.value)}
          />

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

        <div className={style.pagination}>
          <Pagination page={page} totalPages={totalPages} onPageChange={(page) => setPage(page)} />
        </div>
      </main>
    </div>
  )
}

export default Games
