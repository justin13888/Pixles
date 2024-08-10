import { createFileRoute, Link } from '@tanstack/react-router'
import { api } from '@/lib/api';

export const Route = createFileRoute('/albums/')({
  loader: async () => {
    return await api.v1.albums.index.get()
  },
  staleTime: Number.POSITIVE_INFINITY,
  component: () => <Albums />
})

const Albums = () => {
  const albums = Route.useLoaderData()

  return <>
    <div className="grid grid-cols-2">
      {albums.data?.map((album) => (
        <Link to="/albums/$id" params={{ id: album.id}} key={album.id}>
          <div className="rounded-sm bg-background">
            <h2>{album.name}</h2>
            <p>{album.description}</p>
            <img src={album.coverUrl} alt={album.name} />
          </div>
        </Link>
      ))}
    </div>
  </>
}; // TODO: Implement
