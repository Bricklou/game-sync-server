import React from 'react'
import classNames from 'classnames'
import style from './button.module.css'

interface ButtonProps {
  classNames?: string
  onClick?: React.MouseEventHandler<HTMLButtonElement>
  children?: React.ReactNode
  disabled?: boolean
}

function Button(props: ButtonProps): JSX.Element {
  return (
    <button
      className={classNames(props.classNames, style.button)}
      onClick={props.onClick}
      disabled={props.disabled}
    >
      {props.children}
    </button>
  )
}

export default Button
