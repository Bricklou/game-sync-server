import React, { Suspense, useEffect, useState } from 'react'
import { Routes, Route } from 'react-router-dom'
import PrivateRoute from './components/PrivateRoute'
import Loading from './pages/Loading'
import { useAppDispatch, useAppSelector } from './store'
import { refresh } from './store/reducer'

function AppRouter(): JSX.Element {
  const [first, setFirst] = useState(true)
  const auth = useAppSelector((state) => state.auth)
  const dispatch = useAppDispatch()

  const loadLazy = (
    prom: () => Promise<{ default: React.ComponentType<unknown> }>
  ): JSX.Element => {
    const Component = React.lazy(prom)
    return (
      <Suspense fallback={<Loading isLoading={true} />}>
        <Component />
      </Suspense>
    )
  }

  useEffect(() => {
    dispatch(refresh())
    setFirst(false)
  }, [])

  return (
    <Loading isLoading={auth.is_loading && first}>
      <Routes>
        <Route
          path="/login"
          element={loadLazy(() => import(/* webpackChunkName: "login" */ './pages/login'))}
        />
        <Route path="/" element={<PrivateRoute />}>
          <Route element={loadLazy(() => import(/* webpackChunkName: "login" */ './pages/Panel'))}>
            <Route
              path=""
              element={loadLazy(() => import(/* webpackChunkName: "login" */ './pages/panel/Home'))}
            />
          </Route>
        </Route>

        <Route
          path="*"
          element={loadLazy(() => import(/* webpackChunkName: "login" */ './pages/NotFound'))}
        />
      </Routes>
    </Loading>
  )
}

export default AppRouter
