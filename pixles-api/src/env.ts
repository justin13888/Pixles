import { type TypeOf, z } from "zod";

export const envSchema = z.object({
	NODE_ENV: z.string().default("development"),
    PORT: z.string().default("3000"),
    PINO_LOG_LEVEL: z.string().default("info"),
	/**
	 * URL of the MySQL server
	 */
	MYSQL_URL: z.string(),
	// /**
	//  * Random pepper for Argon2id
	//  */
	// ARGON2ID_PEPPER: t.String().min(128),
	// /**
	//  * Memory cost of Argon2id in KiB
	//  */
	// ARGON2ID_MEMORY_COST: t
	// 	.Number({
	// 		minimum: 2 ** 16,
	// 		maximum: 2 ** 32,
	// 	})
	// 	.default(2 ** 16),

	// /**
	//  * Time cost of Argon2id, measured in number of iterations
	//  */
	// ARGON2ID_TIME_COST: t.Number({ minimum: 1, maximum: 10 }).default(3),
});

export type Envs = TypeOf<typeof envSchema>;

const getEnvs = (): Envs => {
    try {
        return envSchema.parse(process.env);
    } catch (err) {
        if (err instanceof z.ZodError) {
            const { fieldErrors } = err.flatten();
            const errorMessage = Object.entries(fieldErrors)
                .map(([field, errors]) =>
                    errors ? `${field}: ${errors.join(", ")}` : field,
                )
                .join("\n  ");
            throw new Error(
                `Missing environment variables:\n  ${errorMessage}`,
            );
        }
        throw err;
    }
};

export const envs = getEnvs();
