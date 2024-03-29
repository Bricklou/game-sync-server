import API, { ApiError } from '@/utils/API'
import { createAsyncThunk, createSlice, PayloadAction } from '@reduxjs/toolkit'
import Axios, { AxiosError, AxiosResponse } from 'axios'

const name = 'auth'

interface AuthState {
  user?: User
  error?: string
  is_error: boolean
  is_authenticated: boolean
  is_loading: boolean
}

const initialState: AuthState = {
  user: undefined,
  error: undefined,
  is_authenticated: false,
  is_loading: false,
  is_error: false
}

export const register = createAsyncThunk<User>(`${name}/register`, async (userData, thunkAPI) => {
  try {
    const response = await API.post('/auth/register', userData)

    if (response.status === 200) {
      return response.data
    } else {
      return thunkAPI.rejectWithValue(response.data)
    }
  } catch (e) {
    thunkAPI.rejectWithValue((e as AxiosResponse).data)
  }
})

export const login = createAsyncThunk<User, { username: string; password: string }>(
  `${name}/login`,
  async (userData, thunkAPI) => {
    try {
      const response = await API.post<User>('/auth', userData)

      if (response.status !== 200) {
        return thunkAPI.rejectWithValue(response.data)
      }

      return response.data
    } catch (e) {
      const error = e as Error | AxiosError<ApiError | undefined>
      if (Axios.isAxiosError(error)) {
        const out = error?.response?.data?.error || error.message
        return thunkAPI.rejectWithValue(out)
      }
      return thunkAPI.rejectWithValue((e as Error).message)
    }
  }
)

export const refresh = createAsyncThunk<User>(`${name}/refresh`, async (userData, thunkAPI) => {
  try {
    const response = await API.get<User>('/auth')

    if (response.status !== 200) {
      return thunkAPI.rejectWithValue(response.data)
    }

    return response.data
  } catch (e) {
    const error = e as Error | AxiosError<ApiError | undefined>
    if (Axios.isAxiosError(error)) {
      const out = error?.response?.data?.error || error.message
      return thunkAPI.rejectWithValue(out)
    }
    return thunkAPI.rejectWithValue((e as Error).message)
  }
})

export const logout = createAsyncThunk<void>(`${name}/logout`, async (userData, thunkAPI) => {
  try {
    const response = await API.delete('/auth')

    if (response.status !== 200) {
      return thunkAPI.rejectWithValue(response.data)
    }

    return response.data
  } catch (e) {
    const error = e as Error | AxiosError<ApiError | undefined>
    if (Axios.isAxiosError(error)) {
      const out = error?.response?.data?.error || error.message
      return thunkAPI.rejectWithValue(out)
    }
    return thunkAPI.rejectWithValue((e as Error).message)
  }
})

export const authSlice = createSlice({
  name,
  initialState,
  reducers: {
    logout: (state) => {
      state.user = undefined
      state.error = undefined
      state.is_authenticated = false
      state.is_loading = false
      state.is_error = false
    },

    clearAuthError: (state) => {
      state.is_error = false
      state.error = undefined
    }
  },
  extraReducers: (builder) => {
    builder.addCase(register.pending, (state) => {
      state.is_loading = true
      state.error = undefined
      state.is_error = false
    })
    builder.addCase(register.fulfilled, (state, action) => {
      state.is_loading = false
      state.user = action.payload
      state.is_authenticated = true
    })
    builder.addCase(register.rejected, (state, action) => {
      state.is_loading = false
      state.user = undefined
      state.is_authenticated = false
      state.error = action.error.message
      state.is_error = true
    })

    builder.addCase(login.pending, (state) => {
      state.is_loading = true
      state.error = undefined
      state.is_error = false
    })
    builder.addCase(login.fulfilled, (state, action: PayloadAction<User>) => {
      state.is_loading = false
      state.user = action.payload
      state.is_authenticated = true
    })
    builder.addCase(login.rejected, (state, action) => {
      state.is_loading = false
      state.user = undefined
      state.is_authenticated = false
      state.error = action.payload as string
      state.is_error = true
    })

    builder.addCase(refresh.pending, (state) => {
      state.is_loading = true
      state.error = undefined
      state.is_error = false
    })
    builder.addCase(refresh.fulfilled, (state, action: PayloadAction<User>) => {
      state.is_loading = false
      state.user = action.payload
      state.is_authenticated = true
    })
    builder.addCase(refresh.rejected, (state, action) => {
      state.is_loading = false
      state.user = undefined
      state.is_authenticated = false
      state.error = action.payload as string
      state.is_error = true
    })

    builder.addCase(logout.pending, (state) => {
      state.is_loading = true
      state.error = undefined
      state.is_error = false
    })
    builder.addCase(logout.fulfilled, (state) => {
      state.is_loading = false
      state.user = undefined
      state.is_authenticated = false
    })
    builder.addCase(logout.rejected, (state, action) => {
      state.is_loading = false
      state.user = undefined
      state.is_authenticated = false
      state.error = action.payload as string
      state.is_error = true
    })
  }
})

export const { clearAuthError } = authSlice.actions

export const userSelector = (state: AuthState): User | undefined => state.user

export default authSlice.reducer
