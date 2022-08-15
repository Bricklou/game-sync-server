import React, { useEffect, useState } from 'react'
import { useParams } from 'react-router-dom'
import style from '@/styles/pages/games/view.module.css'
import API from '@/utils/API'
import LogoUploader from '@/components/games/LogoUploader'

function View(): JSX.Element {
  const params = useParams()
  const [game, setGame] = useState<Game>()
  const [logo, setLogo] = useState<string | null>(null)

  useEffect(() => {
    API.get<GameWithAttachments>(`/games/${params.id}`)
      .then((res) => {
        console.log(res.data)
        setGame(res.data)
        if (res.data.logo) {
          setLogo(`/static/${res.data.logo}`)
        }
      })
      .catch((err) => {
        console.log(err)
      })
  }, [])

  const onImageSelected = async (image: Blob): Promise<[boolean, string]> => {
    try {
      const formData = new FormData()
      formData.append('image', image)
      const resp = await API.put<GameAttachment>(`/games/${params.id}/logo`, formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      })
      return [true, resp.data ? `/static/${resp.data.path}` : '']
    } catch (err) {
      console.error(err)
      return [false, '']
    }
  }

  return (
    <div className={style.game_view}>
      <h1>{game?.name}</h1>

      <LogoUploader
        id="uploader"
        name="uploader"
        onImageSelected={onImageSelected}
        preview={logo}
      />

      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque euismod, nisi eu
        consectetur consectetur, nisl nisl consectetur nisl, eu consectetur nisl nisl euismod nisl.
      </p>
    </div>
  )
}

export default View
