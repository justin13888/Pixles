import { useQuery } from 'urql';
import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/')({
  component: Index,
})

import { graphql } from '@/gql';

const FOO = graphql(`
  query foo($id: ID!) {
    user {
      getUser(id: $id) {
        id
        username
        name
      }
    }
  }
`);

function Index() {
  return (
    <div className="p-2">
      <h3>Welcome Home!</h3>
      <UserInfo />
      {/* TODO */}
    </div>
  )
}

// TODO: Remove this
function UserInfo() {
  const [{ data }] = useQuery({
    query: FOO,
    variables: { id: "sdf" },
  });

  if (!data) return <p>No data</p>;
  return (
    <pre>{JSON.stringify(data, null, 2)}</pre>
  )
}
