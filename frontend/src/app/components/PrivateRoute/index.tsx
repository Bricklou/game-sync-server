import { useAppSelector } from '@/store'
import { RootState } from '@/store/init'
import React from 'react'
import { Navigate, Outlet, useLocation } from 'react-router-dom'

function PrivateRoute(): JSX.Element {
  const auth = useAppSelector((state: RootState) => state?.auth)
  const location = useLocation()

  console.log('is user authenticated ?', auth)

  return auth.is_authenticated ? (
    <Outlet />
  ) : (
    <Navigate to="/login" state={{ from: location }} replace />
  )
}

export default PrivateRoute
