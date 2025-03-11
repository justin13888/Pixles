import { createLazyFileRoute } from "@tanstack/react-router";
import { useMutation, useQuery } from "urql";

export const Route = createLazyFileRoute("/")({
	component: Index,
});

import { graphql } from "@/gql";

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

const BAR = graphql(`
  mutation bar {
    user {
      register(input: {
        name: "hi",
        email: "hi",
        password: "foo"
      }) {
        success
        data {
          token
          user {
            id
            username
          }
        }
        errors
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
	);
}

// TODO: Remove this
function UserInfo() {
	const [{ data, fetching, error }, reexecuteQuery] = useQuery({
		query: FOO,
		variables: { id: "sdf" },
	});
	const [updateUserResult, updateUser] = useMutation(BAR);
	const submit = () => {
		const variables = {};
		updateUser(variables).then((result) => {
			// The result is almost identical to `updateTodoResult` with the exception
			// of `result.fetching` not being set.
			// It is an OperationResult.
			console.log(result);

			if (result.error) {
				console.error(result.error);
			}
		});
	};

	const refresh = () => {
		// Refetch the query and skip the cache
		reexecuteQuery({ requestPolicy: "network-only" });
	};

	if (fetching) return <p>Fetching...</p>;
	if (error) return <p>Error: {error.message}</p>;

	if (!data) return <p>No data</p>;

	return (
		<div>
			<pre>{JSON.stringify(data, null, 2)}</pre>
			<button type="button" onClick={refresh}>
				Refresh
			</button>
		</div>
	);
}
