import React, { Suspense, useEffect, useState } from 'react'
import { Routes, Route, useLocation } from 'react-router-dom'
import PrivateRoute from './components/PrivateRoute'
import Loading from './pages/Loading'
import { useAppDispatch, useAppSelector } from './store'
import { clearAuthError, refresh } from './store/reducer'
import API from './utils/API'

const Login = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "login" */ './pages/login')
)
const Panel = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "panel" */ './pages/Panel')
)
const Home = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "home" */ './pages/panel/Home')
)
const Games = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "games" */ './pages/panel/games/index')
)
const GamesAdd = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "games" */ './pages/panel/games/Add')
)
const GamesView = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "games" */ './pages/panel/games/View')
)
const NotFound = React.lazy(
  () => import(/* webpackPrefetch: true, webpackChunkName: "not_found" */ './pages/NotFound')
)

function AppRouter(): JSX.Element {
  const [first, setFirst] = useState(true)
  const auth = useAppSelector((state) => state.auth)
  const dispatch = useAppDispatch()
  const location = useLocation()

  useEffect(() => {
    API.get('/csrf')
      .then(() => dispatch(refresh()))
      .then(() => location.pathname === '/login' && dispatch(clearAuthError()))
      .then(() => setFirst(false))
  }, [])

  return (
    <Loading isLoading={auth.is_loading && first}>
      <Suspense fallback={<Loading isLoading={true} />}>
        <Routes>
          <Route path="/" element={<PrivateRoute />}>
            <Route element={<Panel />}>
              <Route path="/" element={<Home />} />
              <Route path="/games">
                <Route path="/games" element={<Games />} />
                <Route path="/games/add" element={<GamesAdd />} />
                <Route path="/games/:id" element={<GamesView />} />
              </Route>
            </Route>
          </Route>
          <Route path="/login" element={<Login />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Suspense>
    </Loading>
  )
}

export default AppRouter
