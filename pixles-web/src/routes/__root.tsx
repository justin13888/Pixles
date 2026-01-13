import { Outlet, createRootRoute } from '@tanstack/react-router';

import { AppSidebar } from '@/components/app-sidebar';
import { Header } from '@/components/header';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import React, { Suspense } from 'react';

const queryClient = new QueryClient();

const TanStackRouterDevtools =
    process.env.NODE_ENV === 'production'
        ? () => null // Render nothing in production
        : React.lazy(() =>
              // Lazy load in development
              import('@tanstack/router-devtools').then((res) => ({
                  default: res.TanStackRouterDevtools,
                  // For Embedded Mode
                  // default: res.TanStackRouterDevtoolsPanel
              })),
          );

const ReactQueryDevtoolsProduction = React.lazy(() =>
    import('@tanstack/react-query-devtools/build/modern/production.js').then(
        (d) => ({
            default: d.ReactQueryDevtools,
        }),
    ),
);

export const Route = createRootRoute({
    component: () => {
        const [showDevtools, setShowDevtools] = React.useState(false);

        React.useEffect(() => {
            // @ts-expect-error
            window.toggleDevtools = () => setShowDevtools((old) => !old);
        }, []);

        return (
            <QueryClientProvider client={queryClient}>
                <div className="flex flex-col h-screen">
                    <Header />
                    <div className="flex flex-1 overflow-hidden">
                        <AppSidebar className="hidden md:flex flex-shrink-0" />
                        <main className="flex-1 overflow-y-auto bg-background">
                            <Outlet />
                        </main>
                    </div>
                </div>
                <Suspense>
                    <TanStackRouterDevtools />
                </Suspense>

                <ReactQueryDevtools initialIsOpen={false} />
                {showDevtools && (
                    <React.Suspense fallback={null}>
                        <ReactQueryDevtoolsProduction />
                    </React.Suspense>
                )}
            </QueryClientProvider>
        );
    },
});
