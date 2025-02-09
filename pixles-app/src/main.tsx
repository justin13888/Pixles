import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { Client, Provider, fetchExchange } from 'urql';
import { offlineExchange } from '@urql/exchange-graphcache';
import { makeDefaultStorage } from '@urql/exchange-graphcache/default-storage';
import { requestPolicyExchange } from '@urql/exchange-request-policy';
import { populateExchange } from '@urql/exchange-populate'; import { persistedExchange } from '@urql/exchange-persisted';

import schema from './schema';

// Import the generated route tree
import { routeTree } from './routeTree.gen'

import './index.css'

const exchanges = [];

if (import.meta.env.DEV) {
  await import('@urql/devtools').then(({ devtoolsExchange }) => {
    exchanges.push(devtoolsExchange);
  });
}

const storage = makeDefaultStorage({
  idbName: 'graphcache-v3', // The name of the IndexedDB database
  maxAge: 7, // The maximum age of the persisted data in days
});

exchanges.push(
  populateExchange({
    schema,
  }),
  requestPolicyExchange({
    ttl: 1000 * 60, // 1 minute
  }),
  offlineExchange({
    schema,
    storage,
    // updates: {},
    // optimistic: {},
  }),
  persistedExchange({
    preferGetForPersistedQueries: true,
  }),
  fetchExchange,
);

const client = new Client({
  url: 'http://localhost:3000/graphql',
  exchanges,
});

// const client = new Client({
//   url: 'http://localhost:3000/graphql',
//   exchanges: [cacheExchange, fetchExchange],
//   fetchOptions: () => {
//     const token = getToken();
//     return {
//       headers: { authorization: token ? `Bearer ${token}` : '' },
//     };
//   },
// }); // TODO: Add headers for auth

// Create a new router instance
const router = createRouter({ routeTree })

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

// Render the app
const rootElement = document.getElementById('root')!
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <Provider value={client}>
        <RouterProvider router={router} />
      </Provider>
    </StrictMode>,
  )
}
