import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { Client, Provider, cacheExchange, fetchExchange } from 'urql';

// Import the generated route tree
import { routeTree } from './routeTree.gen'

import './index.css'

const client = new Client({
  url: 'http://localhost:3000/graphql',
  exchanges: [cacheExchange, fetchExchange],
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
