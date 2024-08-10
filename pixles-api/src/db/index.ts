import { drizzle } from "drizzle-orm/mysql2";
import mysql from "mysql2/promise";
import { envs } from "../env";
import * as schema from './schema';

const connection = await mysql.createConnection({
  uri: envs.MYSQL_URL,
});

export const db = drizzle(connection, { schema, mode: 'default' });
