import React from 'react'
import ReactDOMClient from 'react-dom/client'

import App from './App'

const container = document.getElementById('root')

if (container) {
  const root = ReactDOMClient.createRoot(container)
  root.render(React.createElement(App))
} else {
  console.error('Unable to find container root')
}

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
//@ts-ignore
process.env.NODE_ENV === 'development' && module.hot && module.hot.accept()
