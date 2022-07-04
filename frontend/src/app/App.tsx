import React from 'react'
import '@/styles/app.css'
import Button from './components/Button'
import { Provider } from 'react-redux'
import store from './store'

function App(): JSX.Element {
  return (
    <Provider store={store}>
      <div>
        <h1>Hello World !!!</h1>

        <Button>Salut !!!</Button>
      </div>
    </Provider>
  )
}

export default App
