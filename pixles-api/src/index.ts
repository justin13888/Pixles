import { Elysia, error } from "elysia";
import cors from "@elysiajs/cors";
import { v1 } from "./routes/v1";
import swagger from "@elysiajs/swagger";
import serverTiming from "@elysiajs/server-timing";
import { envs } from "./env";

const API_VERSION = "0.1.0";
const version = (version: string) =>
	new Elysia({
		detail: {
			tags: ["About"],
		},
	}).get("/version", `v${version}`); // TODO: May want more details used for client to check version

const app = new Elysia()
	.use(cors())
	.use(serverTiming())
	.use(
		swagger({
			documentation: {
				info: {
					title: "Pixles API",
					version: API_VERSION,
					description: "API for the Pixles application",
				},
				servers: [
					{
						url: "http://localhost:3000",
						description: "Local development server",
					},
				],
        tags: [
			{ name: 'About', description: 'About endpoints' },
			{ name: 'Auth', description: 'Authentication endpoints' },
          { name: 'Album', description: 'Album endpoints' },
        ]
			},
		}),
	)
	.onError(({ code, error }) => {
		// Obfuscate error details
		console.error(code, error); // TODO: replace with logger
		return new Response("Internal Server Error", { status: 500 });
	})
	.use(version(API_VERSION))
	.group("/v1", (app) => app.use(v1()))
	.listen(envs.PORT);

console.log(
	`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`,
);
