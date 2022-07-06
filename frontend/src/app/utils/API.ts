import Axios from 'axios'

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

export default API
