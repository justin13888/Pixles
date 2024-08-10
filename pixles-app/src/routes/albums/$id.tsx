import { api } from '@/lib/api'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/albums/$id')({
  loader: async ({params: {id}}) => {
    return await api.v1.albums({ id: id}).get()
  }, // TODO: Change to react query so it oculd be reused across pages
  staleTime: Number.POSITIVE_INFINITY,
  component: () => <Album />,
})

const Album = () => {
  const album = Route.useLoaderData()

  return <div className="grid">
    {
      album.data?.map((photo) => (
        <li key={photo.id}>
          <img src={photo.thumbnailUrl} alt="thumbnail" />
        </li>
      ))
    }
  </div>
} // TODO: Implement
