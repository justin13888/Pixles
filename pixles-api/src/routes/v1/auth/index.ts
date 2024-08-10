import Elysia, { t } from "elysia";
import { passkeyRoutes } from "./passkey";

export const auth = () => new Elysia({
    detail: {
        tags: ['Auth']
    }
})
    .group('/passkey', (app) => app.use(passkeyRoutes()))


    // .post('/register', ({ body: { email, username, password } }) => {
    //     return {
    //         username,
    //         token: "foo"
    //     } // TODO: Finish implementation
    // }, {
    //     body: t.Object({
    //         email: t.String({ format: 'email' }),
    //         username: t.String({ minLength: 1}),
    //         password: t.String({ minLength: 1}),
    //     }),
    //     response: t.Object({
    //         username: t.String(),
    //         token: t.String(),
    //     }),
    //     // TODO: Define other properties like error
    // })


// TODO: Implement passkey, oauth, then refresh token routes
// TODO: Finish all endpoints here: POST /register, POST /login, POST /logout, POST /oauth/init/:provider, POST /oauth/callback/:provider, POST /oauth/logout, POST /refresh
