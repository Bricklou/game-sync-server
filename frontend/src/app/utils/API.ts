import Axios, { AxiosError } from 'axios'

const API = Axios.create({
  baseURL: '/api',
  timeout: 1000,
  withCredentials: true,
  xsrfHeaderName: 'X-XSRF-TOKEN',
  xsrfCookieName: 'save-sync-csrf'
})

export interface ApiError {
  error: string
}

export function isValidationError<T extends string>(
  error: AxiosError
): error is AxiosError<FormValidationError<T>> {
  return error.response?.status === 400
}

export default API
