import { Area } from 'react-easy-crop'

function createImage(url: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image()
    image.addEventListener('load', () => resolve(image))
    image.addEventListener('error', (error) => reject(error))
    image.setAttribute('crossorigin', 'anonymous')
    image.src = url
  })
}

export async function getCroppedImg(imageSrc: string, pixelCrop: Area): Promise<Blob | null> {
  return new Promise(async (resolve) => {
    const image = await createImage(imageSrc)
    const canvas = document.createElement('canvas')
    const ctx = canvas.getContext('2d')

    if (!ctx) return resolve(null)

    //const maxSize = Math.max(image.width, image.height)

    // set each dimensions to double largest dimension to allow for a safe area for the
    // image to rotate in without being clipped by canvas context
    canvas.width = 300
    canvas.height = 300

    // draw rotated image and store data.
    ctx.drawImage(
      image,
      // SOURCE X, Y, WIDTH, HEIGHT
      pixelCrop.x,
      pixelCrop.y,
      pixelCrop.width,
      pixelCrop.height,
      // DESTINATION X, Y, WIDTH, HEIGHT
      0,
      0,
      300,
      300
    )

    // To blob
    canvas.toBlob((b) => {
      resolve(b)
    }, 'image/jpeg')
  })
}
