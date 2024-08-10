import Elysia from "elysia";
import { auth } from "./auth";
import { albumsRoutes } from "./albums";
import { dashboardRoutes } from "./dashboard";
import { photosRoutes } from "./photos";

export const v1 = () => new Elysia()
  .group('/auth', (app) => app.use(auth()))
  .group('/albums', (app) => app.use(albumsRoutes()))
  .group('/dashboard', (app) => app.use(dashboardRoutes()))
  .group('/photos', (app) => app.use(photosRoutes()));
