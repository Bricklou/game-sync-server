import React, { useCallback, useState } from 'react'
import style from './cropper.module.css'
import EasyCropper from 'react-easy-crop'
import type { Point, MediaSize, Area } from 'react-easy-crop/types'
import Button from '@/components/Button'
import { Crop } from 'react-feather'
import { useEffect } from 'react'
import ReactPortal from '@/components/ReactPortal'
import { getCroppedImg } from './utils'

interface CropperProps {
  getBlob: (blob: Blob | null) => void
  inputImg: File | null
}

function Cropper(props: CropperProps): JSX.Element {
  const [crop, setCrop] = useState<Point>({ x: 0, y: 0 })
  const [area, setArea] = useState<Area>({ x: 0, y: 0, width: 0, height: 0 })
  const [zoom, setZoom] = useState(1)
  const [minZoom, setMinZoom] = useState(1)

  const [previewUrl, setPreviewUrl] = useState<string | undefined>(undefined)

  useEffect(() => {
    if (!props.inputImg) return

    const reader = new FileReader()
    reader.addEventListener('load', () =>
      setPreviewUrl((reader.result ?? undefined) as string | undefined)
    )
    reader.readAsDataURL(props.inputImg)
  }, [props.inputImg])

  const onMediaLoaded = useCallback((media: MediaSize) => {
    const minSize = Math.min(media.width, media.height)
    setMinZoom(300 / minSize)
    setZoom(minZoom)
  }, [])

  const finish = async (): Promise<void> => {
    if (!previewUrl) return props.getBlob(null)
    const croppedImage = await getCroppedImg(previewUrl, area)
    props.getBlob(croppedImage)
  }

  return (
    <ReactPortal wrapperId="portal-cropper-modal">
      <div className={style.cropper}>
        <div className={style.container}>
          <EasyCropper
            crop={crop}
            onCropChange={setCrop}
            image={previewUrl}
            zoom={zoom}
            aspect={1}
            onCropComplete={(croppedArea, croppedAreaPixel) => setArea(croppedAreaPixel)}
            onZoomChange={setZoom}
            cropSize={{ width: 300, height: 300 }}
            objectFit="horizontal-cover"
            onMediaLoaded={onMediaLoaded}
            classes={{
              containerClassName: style.cropper_container,
              cropAreaClassName: style.cropper_crop_area
            }}
            minZoom={minZoom}
          />
          <div className={style.actions}>
            <Button onClick={() => props.getBlob(null)}>Cancel</Button>
            <Button onClick={finish}>
              <Crop />
              Crop
            </Button>
          </div>
        </div>
      </div>
    </ReactPortal>
  )
}

export default Cropper
