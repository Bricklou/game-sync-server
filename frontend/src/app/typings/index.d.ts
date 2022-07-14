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
