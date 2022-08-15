import React, { useLayoutEffect } from 'react'
import { createPortal } from 'react-dom'

interface ReactPortalProps {
  children?: React.ReactNode
  wrapperId: string
}

function createWrapperAndAppendToBody(wrapperId: string): HTMLDivElement {
  const wrapperEl = document.createElement('div')
  wrapperEl.id = wrapperId
  document.body.appendChild(wrapperEl)
  return wrapperEl
}

function ReactPortal({
  children,
  wrapperId = 'react-portal-wrapper'
}: ReactPortalProps): JSX.Element | null {
  const [wrapperEl, setWrapperEl] = React.useState<HTMLElement | null>(null)

  useLayoutEffect(() => {
    let el = document.getElementById(wrapperId)
    let systemCreated = false

    // if the element is not found with wrapperId or wrapperId is not provided, create and append to body
    if (!el) {
      systemCreated = true
      el = createWrapperAndAppendToBody(wrapperId)
      setWrapperEl(el)
    }

    return () => {
      // delete the programmatically created element
      if (systemCreated && el?.parentNode) {
        el.parentNode.removeChild(el)
      }
    }
  }, [wrapperId])

  if (!wrapperEl) return null

  return createPortal(children, wrapperEl)
}

export default ReactPortal
