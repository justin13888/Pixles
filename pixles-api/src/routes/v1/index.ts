import Elysia from "elysia";
import { auth } from "./auth";
import { albums } from "./albums";
import { dashboard } from "./dashboard";

export const v1 = () => new Elysia()
  .group('/auth', (app) => app.use(auth()))
  .group('/albums', (app) => app.use(albums()))
  .group('/dashboard', (app) => app.use(dashboard()));
