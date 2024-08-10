import { envs } from "@/env";
import { DATABASE_PREFIX } from "@/lib/constants";
import type { Config } from "drizzle-kit";

export default {
    schema: "./src/db/schema.ts",
    dialect: "mysql",
    out: "./drizzle",
    dbCredentials: {
        url: envs.MYSQL_URL,
    },
    tablesFilter: [`${DATABASE_PREFIX}_*`],
} satisfies Config;
