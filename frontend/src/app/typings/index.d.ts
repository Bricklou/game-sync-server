declare module '*.module.css'

declare const __API__: string

interface User {
  username: string
}

type FormValidationError<T extends string> = {
  message: string
  fields: {
    [key in T]: string[]
  }
}

interface Game {
  id: string
  name: string
  link: string | null
  author: string
  created_at: Date
  updated_at: Date
}

interface Paginated<T> {
  page: number
  per_page: number
  total: number
  total_pages: number
  items: T[]
}
