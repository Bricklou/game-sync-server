import classNames from 'classnames'
import React, { forwardRef } from 'react'
import style from './select.module.css'

interface SelectOptions {
  value: string
  label: string
  selected?: boolean
  disabled?: boolean
}

interface SelectProps {
  id: string
  name: string
  className?: string
  value?: string
  defaultValue?: string
  onChange?: React.ChangeEventHandler<HTMLSelectElement>
  onBlur?: React.FocusEventHandler<HTMLSelectElement>
  placeholder?: string
  disabled?: boolean
  error?: string
  required?: boolean
  label?: string
  selected?: string

  options: SelectOptions[]
}

function Select(props: SelectProps, ref: React.Ref<HTMLSelectElement>): JSX.Element {
  return (
    <div className={style.select_container}>
      {props.label && <label htmlFor={props.id}>{props.label}</label>}

      <select
        id={props.id}
        name={props.name}
        className={classNames(style.select, props.className)}
        value={props.value}
        defaultValue={props.defaultValue}
        onChange={props.onChange}
        onBlur={props.onBlur}
        placeholder={props.placeholder}
        disabled={props.disabled}
        required={props.required}
        ref={ref}
      >
        {props.options.map((option) => (
          <option key={option.value} value={option.value} disabled={option.disabled}>
            {option.label}
          </option>
        ))}
      </select>

      {props.error && <p className={style.error}>{props.error}</p>}
    </div>
  )
}

export default forwardRef(Select)
