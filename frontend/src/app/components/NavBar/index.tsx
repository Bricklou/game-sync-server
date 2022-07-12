import { useAppDispatch } from '@/store'
import { logout } from '@/store/reducer'
import classNames from 'classnames'
import React from 'react'
import { NavLink } from 'react-router-dom'
import Button from '../Button'
import style from './navbar.module.css'

function NavBarLink(props: { to: string; children: React.ReactNode }): JSX.Element {
  return (
    <NavLink
      className={({ isActive }) => classNames(style.link, { [style.active]: isActive })}
      to={props.to}
    >
      {props.children}
    </NavLink>
  )
}

function NavBar(): JSX.Element {
  const dispatch = useAppDispatch()

  const logout_action = (): void => {
    dispatch(logout())
  }
  return (
    <nav className={style.navbar}>
      <div className={style.container}>
        <div className={style.content}>
          <div className={style.left}>
            <NavLink to="/" className={style.title}>
              Save Sync
            </NavLink>

            <div className={style.links_container}>
              <div className={style.links}>
                <NavBarLink to="/">Home</NavBarLink>
                <NavBarLink to="/games">Games</NavBarLink>
                <NavBarLink to="/saves">Saves</NavBarLink>
              </div>
            </div>
          </div>
          <div className={style.right}>
            <Button onClick={logout_action}>Logout</Button>
          </div>
        </div>
      </div>
    </nav>
  )
}

export default NavBar
