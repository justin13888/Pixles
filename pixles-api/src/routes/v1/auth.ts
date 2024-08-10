import Elysia, { t } from "elysia";

export const auth = () => new Elysia({
    detail: {
        tags: ['Auth']
    }
})
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

// TODO: Finish all endpoints here: POST /register, POST /login, POST /logout, POST /oauth/init/:provider, POST /oauth/callback/:provider, POST /oauth/logout, POST /refresh
// POST /passkey/register, POST /passkey/login, POST /passkey/refresh
