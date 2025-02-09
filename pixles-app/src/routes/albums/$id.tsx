import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/albums/$id')({
  staleTime: Number.POSITIVE_INFINITY,
  component: () => <Album />,
})

const Album = () => {
  return <div>Album</div>;
  // const album = Route.useLoaderData()

  // return <div className="grid">
  //   {
  //     album.data?.map((photo) => (
  //       <li key={photo.id}>
  //         <img src={photo.thumbnailUrl} alt="thumbnail" />
  //       </li>
  //     ))
  //   }
  // </div>
} // TODO: Implement
