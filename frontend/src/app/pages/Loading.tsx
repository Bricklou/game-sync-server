import React from 'react'
import style from '@/styles/pages/loading.module.css'
import classNames from 'classnames'

interface LoadingProps {
  isLoading: boolean
  children: React.ReactNode
}

function Loading(props: LoadingProps): JSX.Element {
  return (
    <div className={style.container}>
      <div className={classNames(style.loading, { [style.active]: props.isLoading })}>
        <h1>Loading</h1>
      </div>
      {props.children}
    </div>
  )
}

export default Loading
