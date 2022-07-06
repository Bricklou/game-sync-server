import authReducer from './auth.slice'
import usersReducer from './users.slice'

export * from './auth.slice'
export * from './users.slice'

const reducers = {
  auth: authReducer,
  user: usersReducer
}

export default reducers
