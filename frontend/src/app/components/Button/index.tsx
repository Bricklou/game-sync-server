import React from 'react'
import classNames from 'classnames'
import './button.css'

interface ButtonProps {
  classNames?: string
  onClick?: React.MouseEventHandler<HTMLButtonElement>
  children?: React.ReactNode
}

function Button(props: ButtonProps): JSX.Element {
  return (
    <button className={classNames(props.classNames)} onClick={props.onClick}>
      {props.children}
    </button>
  )
}

export default Button
