import Elysia, { t } from "elysia";

export const passkeyRoutes = () => new Elysia()
    .group('/register', (app) => app
        .post('/start', ({ body: { username }}) => {
            return {
                challenge: "foo",
                user: {
                    id: "user_id",
                    username,
                },
                rp: {
                    name: "Pixles",
                }
            }
        }, {
            body: t.Object({
                username: t.String(),
            }),
            response: t.Object({
                challenge: t.String(),
                user: t.Object({
                    id: t.String(),
                    username: t.String(),
                }),
                rp: t.Object({
                    name: t.String(),
                }),
            }),
        })
        .post('/complete', ({ body }) => {
            return {
                id: "cred_id",
                rawId: "raw_id",
                response: {
                    clientDataJSON: "base64_encoded_client_data_json",
                    attestationObject: "base64_encoded_attestation_object",
                },
                type: "public-key",
            }
        }, {
            body: t.Object({
                id: t.String(),
                rawId: t.String(),
                response: t.Object({
                    clientDataJSON: t.String(),
                    attestationObject: t.String(),
                }),
                type: t.Literal('public-key'),
            })
        })
    ).group('/login', (app) => app
        .post('/start', ({ body: { username }}) => {
            return {
                challenge: "foo",
                allowCredentials: [
                    {
                        id: "cred_id",
                        type: "public-key",
                    }
                ],
                userVerification: "required",
            }
        }, {
            body: t.Object({
                username: t.String(),
            }),
            response: t.Object({
                challenge: t.String(),
                allowCredentials: t.Array(t.Object({
                    id: t.String(),
                    type: t.Literal('public-key'),
                })),
                userVerification: t.Literal('required'),
            }),
        })
        .post('/complete', ({ body }) => {
            // body = {
            //     id: "cred_id",
            //     rawId: "raw_id",
            //     response: {
            //         clientDataJSON: "base64_encoded_client_data_json",
            //         authenticatorData: "base64_encoded_authenticator_data",
            //         signature: "base64_encoded_signature",
            //         userHandle: "base64_encoded_user_handle",
            //     },
            //     type: "public-key",
            // }
            return {
                status: "ok",
                token: "session_token", // TODO
            }
        }, {
            body: t.Object({
                id: t.String(),
                rawId: t.String(),
                response: t.Object({
                    clientDataJSON: t.String(),
                    authenticatorData: t.String(),
                    signature: t.String(),
                    userHandle: t.String(),
                }),
                type: t.Literal('public-key'),
            }, {
                response: t.Object({
                    status: t.Literal('ok'),
                    token: t.String(),
                }),
            })
        })
    )
    .delete('/credential', ({ body: { credentialId } }) => {

    }, {
        detail: {
            summary: "Revoke passkey credential"
        },
        body: t.Object({
            credentialId: t.String(),
        }),
        response: t.Undefined({ description: "Success" })
    })
// TODO: Check if schemas are well designed
// TODO: Implement all endpoints
