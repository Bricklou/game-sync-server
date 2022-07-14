import React, { useEffect } from 'react'
import Button from '../components/Button'

import style from '@/styles/pages/login.module.css'
import Input from '../components/Input'
import { useLocation, useNavigate } from 'react-router-dom'
import { login, clearAuthError } from '@/store/reducer'
import { Controller, useForm } from 'react-hook-form'
import { yupResolver } from '@hookform/resolvers/yup'
import * as yup from 'yup'
import { X, AlertTriangle } from 'react-feather'
import { useAppDispatch, useAppSelector } from '@/store'

interface IFormValues {
  username: string
  password: string
}

interface RouterRedirect {
  from: Location
}

const schema = yup
  .object()
  .shape({
    username: yup.string().required().defined(),
    password: yup.string().required().defined()
  })
  .required()

function Login(): JSX.Element {
  const {
    clearErrors,
    handleSubmit,
    control,
    formState: { errors, isValid, isDirty }
  } = useForm<IFormValues>({
    resolver: yupResolver(schema),
    mode: 'onChange',
    shouldFocusError: true,
    defaultValues: {
      username: '',
      password: ''
    }
  })

  const auth = useAppSelector((state) => state?.auth)
  const dispatch = useAppDispatch()

  const navigate = useNavigate()
  const location = useLocation()

  const redirectPath = (location.state as RouterRedirect | undefined)?.from.pathname || '/'

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

  useEffect(() => {
    console.log('isDirty', isDirty)
    console.log('isValid', isValid)
  }, [isDirty, isValid])

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

          <Controller
            name="username"
            control={control}
            rules={{ required: true }}
            render={({ field }) => (
              <Input
                id="username"
                placeholder="Username"
                disabled={auth.is_loading}
                error={errors.username?.message}
                {...field}
              />
            )}
          />

          <Controller
            name="password"
            control={control}
            rules={{ required: true }}
            render={({ field }) => (
              <Input
                id="password"
                type="password"
                placeholder="Password"
                disabled={auth.is_loading}
                error={errors.password?.message}
                {...field}
              />
            )}
          />

          <Button
            classNames={style.submit_button}
            disabled={auth.is_loading || !isValid || !isDirty}
          >
            Log in
          </Button>
        </form>
      </div>
    </div>
  )
}

export default Login
