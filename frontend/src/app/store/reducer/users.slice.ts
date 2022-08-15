import { createSlice } from '@reduxjs/toolkit'

interface UsersState {
  username: string
}

const initialState: UsersState = {
  username: ''
}

export const usersSlice = createSlice({
  name: 'users',
  initialState,
  reducers: {}
})

export const {} = usersSlice.actions

export default usersSlice.reducer
