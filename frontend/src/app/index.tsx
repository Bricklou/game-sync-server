import React from 'react'
import ReactDOMClient from 'react-dom/client'

import App from './App'

const root = ReactDOMClient.createRoot(document.getElementById('root') as HTMLDivElement)
root.render(<App />)

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
//@ts-ignore
process.env.NODE_ENV === 'development' && module.hot && module.hot.accept()
