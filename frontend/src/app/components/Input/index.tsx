import classNames from 'classnames'
import React, { forwardRef } from 'react'
import style from './input.module.css'

interface InputProps {
  id: string
  name: string
  className?: string
  type?: React.HTMLInputTypeAttribute
  value?: string
  onChange?: React.ChangeEventHandler<HTMLInputElement>
  onBlur?: React.FocusEventHandler<HTMLInputElement>
  placeholder?: string
  disabled?: boolean
  error?: string
  required?: boolean
  label?: string
}

function Input(props: InputProps, ref: React.Ref<HTMLInputElement>): JSX.Element {
  return (
    <div className={style.input_container}>
      {props.label && <label htmlFor={props.id}>{props.label}</label>}
      <input
        id={props.id}
        name={props.name}
        className={classNames(style.input, props.className)}
        type={props.type || 'text'}
        value={props.value}
        onChange={props.onChange}
        onBlur={props.onBlur}
        placeholder={props.placeholder}
        disabled={props.disabled}
        required={props.required}
        ref={ref}
      />

      {props.error && <p className={style.error}>{props.error}</p>}
    </div>
  )
}

export default forwardRef(Input)
