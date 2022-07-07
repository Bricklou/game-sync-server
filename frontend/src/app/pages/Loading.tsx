import React, { useRef } from 'react'
import style from '@/styles/pages/loading.module.css'
import classNames from 'classnames'
import { CSSTransition } from 'react-transition-group'

interface LoadingProps {
  isLoading: boolean
  children?: React.ReactNode
}

function Loading(props: LoadingProps): JSX.Element {
  const ref = useRef<HTMLDivElement>(null)
  return (
    <div className={style.container}>
      <CSSTransition
        in={props.isLoading}
        classNames={{
          exitActive: style.transition_exit_active
        }}
        timeout={601}
        unmountOnExit
        nodeRef={ref}
      >
        <div ref={ref} className={classNames(style.loading, { [style.active]: props.isLoading })}>
          <h1>Loading</h1>
        </div>
      </CSSTransition>
      {props.children}
    </div>
  )
}

export default Loading
