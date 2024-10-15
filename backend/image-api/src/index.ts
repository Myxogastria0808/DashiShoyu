import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { prettyJSON } from "hono/pretty-json";
import { cors } from "hono/cors";
import { item } from "./routes/item";
import { object } from "./routes/object";

const app = new Hono().basePath("/api/image");

app.use(prettyJSON());

app.use(
  "/*",
  cors({
    origin: ["http://localhost:3000"],
    allowHeaders: ["Content-Type"],
    allowMethods: ["POST", "DELETE"],
    exposeHeaders: ["Content-Type"],
    credentials: true,
  })
);

app.route("/item", item);
app.route("/object", object);

const port = 7000;
console.log(`Server is running on port: http://localhost:${port}`);

serve({
  fetch: app.fetch,
  port,
});
