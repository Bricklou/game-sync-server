import React from 'react'
import style from './pagination.module.css'
import { ArrowLeft, ArrowRight } from 'react-feather'
import classNames from 'classnames'

interface PaginationProps {
  page: number
  totalPages: number
  onPageChange: (page: number) => void

  siglingCount?: number
}

function Pagination(props: PaginationProps): JSX.Element {
  return (
    <ul className={style.pagination}>
      {props.page > 1 && (
        <li className={style.item} onClick={() => props.onPageChange(props.page - 1)}>
          <ArrowLeft />
        </li>
      )}

      {Array.from({ length: props.totalPages }, (_, i) => {
        const current = i + 1
        return (
          <li
            key={current}
            className={classNames(style.item, { [style.active_item]: current === props.page })}
            onClick={() => props.onPageChange(current)}
          >
            {current}
          </li>
        )
      })}

      {props.page < props.totalPages && (
        <li className={style.item} onClick={() => props.onPageChange(props.page + 1)}>
          <ArrowRight />
        </li>
      )}
    </ul>
  )
}

export default Pagination
