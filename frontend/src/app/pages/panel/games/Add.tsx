import React from 'react'
import style from '@/styles/pages/games/add.module.css'
import { ArrowLeft, Save } from 'react-feather'
import { Link } from 'react-router-dom'
import Input from '@/components/Input'
import Button from '@/components/Button'
import { useForm } from 'react-hook-form'
import API from '@/utils/API'
import Axios, { AxiosError } from 'axios'

interface IFormValues {
  name: string
  link: string
  author: string
}

function Add(): JSX.Element {
  const {
    register,
    formState: { errors },
    handleSubmit
  } = useForm<IFormValues>()

  const onSubmit = async (data: IFormValues): Promise<void> => {
    try {
      const resp = await API.post('/games', data)
      console.log(resp)
    } catch (error) {
      if (Axios.isAxiosError(error)) {
        const { response } = error as AxiosError
        console.log(response?.data)
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
        <Input
          id="name"
          label="Game Name"
          placeholder="Game Name"
          required={true}
          {...register('name', {
            required: true
          })}
          error={errors.name?.message}
        />

        <Input
          id="link"
          label="Game Link"
          placeholder="Game Link"
          required={true}
          {...register('link', {
            required: true
          })}
          error={errors.link?.message}
        />

        <Input
          id="author"
          label="Game Author"
          placeholder="Game Author"
          required={true}
          {...register('author', {
            required: true
          })}
          error={errors.link?.message}
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
