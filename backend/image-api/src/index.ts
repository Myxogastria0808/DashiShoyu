import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { prettyJSON } from "hono/pretty-json";
import { cors } from "hono/cors";
import { item } from "./routes/item";

const app = new Hono().basePath("/api");

app.use(prettyJSON());

app.use(
  "/*",
  cors({
    origin: ["http://localhost:3000"],
    allowHeaders: ["Content-Type"],
    allowMethods: ["POST"],
    exposeHeaders: ["Content-Type"],
    credentials: true,
  })
);

app.route("/item", item);

const port = 7000;
console.log(`Server is running on port: http://localhost:${port}`);

serve({
  fetch: app.fetch,
  port,
});
