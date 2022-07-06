import React, { useEffect } from 'react'
import { Routes, Route } from 'react-router-dom'
import PrivateRoute from './components/PrivateRoute'
import Login from './pages/login'
import { useAppDispatch, useAppSelector } from './store'
import { refresh } from './store/reducer'

function AppRouter(): JSX.Element {
  const auth = useAppSelector((state) => state.auth)
  const dispatch = useAppDispatch()

  useEffect(() => {
    dispatch(refresh())
  }, [])

  if (auth.is_loading) {
    return <h1>Loading !!!</h1>
  }

  return (
    <Routes>
      <Route path="/login" element={<Login />} />
      <Route path="/" element={<PrivateRoute />}>
        <Route path="/" element={<h1>Home !!</h1>} />
      </Route>

      <Route path="*" />
    </Routes>
  )
}

export default AppRouter
