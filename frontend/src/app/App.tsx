import React from 'react'
import { Provider } from 'react-redux'
import store from './store'
import AppRouter from './router'
import { BrowserRouter } from 'react-router-dom'

import '@/styles/app.css'

function App(): JSX.Element {
  return (
    <React.StrictMode>
      <Provider store={store}>
        <BrowserRouter>
          <AppRouter />
        </BrowserRouter>
      </Provider>
    </React.StrictMode>
  )
}

export default App
