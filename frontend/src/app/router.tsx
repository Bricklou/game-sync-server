import React, { useEffect } from 'react'
import { Routes, Route } from 'react-router-dom'
import PrivateRoute from './components/PrivateRoute'
import Loading from './pages/Loading'
import Login from './pages/login'
import { useAppDispatch, useAppSelector } from './store'
import { refresh } from './store/reducer'

function AppRouter(): JSX.Element {
  const auth = useAppSelector((state) => state.auth)
  const dispatch = useAppDispatch()

  useEffect(() => {
    dispatch(refresh())
  }, [])

  return (
    <Loading isLoading={auth.is_loading}>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/" element={<PrivateRoute />}>
          <Route path="/" element={<h1>Home !!</h1>} />
        </Route>

        <Route path="*" />
      </Routes>
    </Loading>
  )
}

export default AppRouter
