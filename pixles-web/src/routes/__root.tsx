import { Outlet, createRootRoute } from "@tanstack/react-router";

import { Header } from "@/components/header";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import React, { Suspense } from "react";

const queryClient = new QueryClient();

const TanStackRouterDevtools =
	process.env.NODE_ENV === "production"
		? () => null // Render nothing in production
		: React.lazy(() =>
				// Lazy load in development
				import("@tanstack/router-devtools").then((res) => ({
					default: res.TanStackRouterDevtools,
					// For Embedded Mode
					// default: res.TanStackRouterDevtoolsPanel
				})),
			);

const ReactQueryDevtoolsProduction = React.lazy(() =>
	import("@tanstack/react-query-devtools/build/modern/production.js").then(
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
				<Header />
				{/* TODO: Add alert card here (e.g. storage limit reached, user errors, concerning activity) */}
				<Outlet />
				<Suspense>
					<TanStackRouterDevtools />
				</Suspense>

				<ReactQueryDevtools initialIsOpen />
				{showDevtools && (
					<React.Suspense fallback={null}>
						<ReactQueryDevtoolsProduction />
					</React.Suspense>
				)}
			</QueryClientProvider>
		);
	},
});
