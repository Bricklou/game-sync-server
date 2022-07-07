import NavBar from '@/components/NavBar'
import React from 'react'
import { Outlet } from 'react-router-dom'

function Panel(): JSX.Element {
  return (
    <div>
      <NavBar />
      <Outlet />
    </div>
  )
}

export default Panel
