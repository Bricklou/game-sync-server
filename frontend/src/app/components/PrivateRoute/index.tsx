import React from 'react'
import { useAppSelector } from '@/store'
import { RootState } from '@/store/init'
import { Navigate, Outlet, useLocation } from 'react-router-dom'

function PrivateRoute(): JSX.Element | null {
  const auth = useAppSelector((state: RootState) => state?.auth)
  const location = useLocation()

  return auth.is_authenticated ? (
    <Outlet />
  ) : (
    <Navigate to="/login" state={{ from: location }} replace />
  )
}

export default PrivateRoute
