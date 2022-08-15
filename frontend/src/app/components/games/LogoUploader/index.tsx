import React, { HTMLAttributes, useEffect } from 'react'
import { CameraOff } from 'react-feather'
import style from './logo_uploader.module.css'
import classNames from 'classnames'
import Cropper from './Cropper'
//import Cropper from './Cropper'

type LogoUploaderProps = {
  onImageSelected?: (image: Blob) => Promise<[boolean, string]> | [boolean, string]
  id: string
  name: string
  preview: string | null
} & HTMLAttributes<HTMLDivElement>

function LogoUploader(props: LogoUploaderProps): JSX.Element {
  const inputRef = React.useRef<HTMLInputElement>(null)
  const [hovered, setHovered] = React.useState(false)

  const [imageFile, setFile] = React.useState<File | null>(null)

  const [previewUrl, setPreviewUrl] = React.useState<string | null>(props.preview)
  useEffect(() => setPreviewUrl(props.preview), [props.preview])

  const isImageFile = (file: File): boolean => file.type.startsWith('image/')

  const handleOnDragOver = (e: React.DragEvent<HTMLDivElement>): void => {
    e.preventDefault()
    e.stopPropagation()

    const file = e.dataTransfer.files[0]
    if (!file) return

    setHovered(isImageFile(file))

    console.log(e)
  }

  const handleOnDrop = (e: React.DragEvent<HTMLDivElement>): void => {
    e.preventDefault()
    e.stopPropagation()

    const file = e.dataTransfer.files[0]
    if (!file || !isImageFile(file)) return

    setFile(file)
  }

  const handleOnClicked = (e: React.MouseEvent<HTMLDivElement>): void => {
    e.preventDefault()
    e.stopPropagation()

    inputRef.current?.click()
  }

  const onUpdate = async (blob: Blob | null): Promise<void> => {
    if (blob && props.onImageSelected) {
      const [isOk, url] = await props.onImageSelected(blob)
      if (isOk) {
        setPreviewUrl(url)
      }
    }
    setFile(null)
  }

  return (
    <div className={classNames(style.logo_uploader, props.className)}>
      <div
        className={classNames(style.drop_zone, { [style.active]: hovered })}
        onDragOver={handleOnDragOver}
        onDrop={handleOnDrop}
        onClick={handleOnClicked}
      >
        <div className={style.image}>{previewUrl ? <img src={previewUrl} /> : <CameraOff />}</div>

        <p>Drag and drop image here</p>
      </div>
      {imageFile && <Cropper getBlob={onUpdate} inputImg={imageFile} />}
      <input
        type="file"
        accept="image/*"
        name={props.name}
        id={props.id}
        ref={inputRef}
        onChange={(e) => setFile(e.target.files?.[0] ?? null)}
      />
    </div>
  )
}

export default LogoUploader
