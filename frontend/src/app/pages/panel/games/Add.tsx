import React from 'react'
import style from '@/styles/pages/games/add.module.css'
import { ArrowLeft, Save } from 'react-feather'
import { Link, useNavigate } from 'react-router-dom'
import Input from '@/components/Input'
import Button from '@/components/Button'
import { Controller, useForm } from 'react-hook-form'
import API, { isValidationError } from '@/utils/API'
import Axios from 'axios'
import { yupResolver } from '@hookform/resolvers/yup'
import * as yup from 'yup'

interface IFormValues {
  name: string
  link: string
  author: string
}

const schema = yup
  .object()
  .shape({
    name: yup.string().required().defined(),
    link: yup.string().url().defined(),
    author: yup.string().required().defined()
  })
  .required()

function Add(): JSX.Element {
  const {
    control,
    setError,
    formState: { errors },
    handleSubmit
  } = useForm<IFormValues>({
    resolver: yupResolver(schema),
    mode: 'onChange',
    shouldFocusError: true,
    defaultValues: {
      name: '',
      author: '',
      link: ''
    }
  })

  const navigate = useNavigate()

  const onSubmit = async (data: IFormValues): Promise<void> => {
    try {
      await API.post('/games', data)

      navigate('/games')
    } catch (error) {
      if (Axios.isAxiosError(error)) {
        if (isValidationError<keyof IFormValues>(error)) {
          const { response } = error

          if (response?.data) {
            for (const [key, value] of Object.entries(response?.data.fields)) {
              setError(key as keyof IFormValues, {
                type: 'validate',
                message: value[0]
              })
            }
          }
        } else if (error.response?.status === 409) {
          setError('name', {
            type: 'validate',
            message: 'Game already exists'
          })
        }
      }
      console.error(error)
    }
  }

  return (
    <div className={style.add}>
      <header>
        <Link to="/games" className={style.back}>
          <ArrowLeft />
        </Link>
        <h1>Add a game</h1>
      </header>

      <form onSubmit={handleSubmit(onSubmit)}>
        <Controller
          name="name"
          control={control}
          rules={{ required: true }}
          render={({ field }) => (
            <Input
              id="name"
              label="Game Name"
              placeholder="Game Name"
              error={errors.name?.message}
              {...field}
            />
          )}
        />

        <Controller
          name="link"
          control={control}
          render={({ field }) => (
            <Input
              id="link"
              label="Game Link"
              placeholder="Game Link"
              error={errors.link?.message}
              {...field}
            />
          )}
        />

        <Controller
          name="author"
          control={control}
          rules={{ required: true }}
          render={({ field }) => (
            <Input
              id="author"
              label="Game Author"
              placeholder="Game Author"
              error={errors.author?.message}
              {...field}
            />
          )}
        />

        <Button classNames={style.add_btn}>
          <Save />
          Save
        </Button>
      </form>
    </div>
  )
}

export default Add
