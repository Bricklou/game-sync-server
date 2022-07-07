import Button from '@/components/Button'
import React from 'react'
import { useNavigate } from 'react-router-dom'
import style from '@/styles/pages/not_found.module.css'

function NotFound(): JSX.Element {
  const navigate = useNavigate()
  return (
    <div className={style.not_found}>
      <h1>Page not found</h1>

      <Button onClick={() => navigate('/')}>Go back to homepage</Button>
    </div>
  )
}

export default NotFound
