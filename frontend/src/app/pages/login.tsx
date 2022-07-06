import React, { useEffect } from 'react'
import Button from '../components/Button'

import style from '@/styles/pages/login.module.css'
import Input from '../components/Input'
import { useLocation, useNavigate } from 'react-router-dom'
import { login, clearAuthError } from '@/store/reducer'
import { useForm } from 'react-hook-form'
import { yupResolver } from '@hookform/resolvers/yup'
import * as yup from 'yup'
import { X, AlertTriangle } from 'react-feather'
import { useAppDispatch, useAppSelector } from '@/store'

interface IFormValues {
  username: string
  password: string
}

const schema = yup
  .object({
    username: yup.string().required().defined(),
    password: yup.string().required().defined()
  })
  .required()

function Login(): JSX.Element {
  const {
    register,
    clearErrors,
    handleSubmit,
    formState: { errors }
  } = useForm<IFormValues>({ resolver: yupResolver(schema) })

  const auth = useAppSelector((state) => state?.auth)
  const dispatch = useAppDispatch()

  const navigate = useNavigate()
  const location = useLocation()

  const redirectPath = (location.state as { path?: string })?.path || '/'

  useEffect(() => {
    if (auth.is_error) {
      console.error('error !! ', auth.error)
    } else if (auth.is_authenticated) {
      navigate(redirectPath, { replace: true })
    }
  }, [auth])

  const onSubmit = (data: IFormValues): void => {
    clearErrors()
    dispatch(login(data))
  }

  return (
    <div className={style.login}>
      <div className={style.sidebar}>
        <form onSubmit={handleSubmit(onSubmit)}>
          <h2 className={style.title}>SaveSync</h2>

          {auth.is_error && (
            <div className={style.error_box}>
              <AlertTriangle />
              <p>{auth.error}</p>
              <X className={style.icon} onClick={() => dispatch(clearAuthError())} />
            </div>
          )}

          <Input
            id="username"
            placeholder="Username"
            disabled={auth.is_loading}
            error={errors.username?.message}
            {...register('username', {
              required: true
            })}
          />

          <Input
            id="username"
            type="password"
            placeholder="Password"
            disabled={auth.is_loading}
            error={errors.password?.message}
            {...register('password', {
              required: true
            })}
          />

          <Button classNames={style.submit_button} disabled={auth.is_loading}>
            Log in
          </Button>
        </form>
      </div>
    </div>
  )
}

export default Login
