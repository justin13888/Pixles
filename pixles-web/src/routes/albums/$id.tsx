import { createFileRoute } from '@tanstack/react-router';
import { graphql } from 'graphql';

export const Route = createFileRoute('/albums/$id')({
    staleTime: Number.POSITIVE_INFINITY,
    component: () => <Album />,
});

// const AlbumFragment = graphql(`
//   fragment AlbumFragment on Album {
//     id
//     title
//   }
// `);

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
}; // TODO: Implement
