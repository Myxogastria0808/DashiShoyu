import { Hono } from "hono";
import { HTTPException } from "hono/http-exception";
import sharp from "sharp";
import { uploadImage } from "../utils/r2";

const item = new Hono();

item.post("/upload/:id", async (c) => {
  console.log("aaaaaaa");
  const id: string = c.req.param("id");
  const body: {
    [x: string]: string | File;
  } = await c.req.parseBody();
  const file: string | File = body["file"];
  if (typeof file === "string") {
    throw new HTTPException(401, { message: "Invald file type" });
  } else {
    try {
      let buffer: Buffer = Buffer.from(await file.arrayBuffer());
      let webp: Buffer = await sharp(buffer)
        .toFormat("webp", { quality: 75 })
        .toBuffer();
      await uploadImage(`${id}.webp`, webp);
      console.log(`[INFO]: ${id}.webp was updated (date: ${new Date()})`);
      c.status(200);
      return c.json({ message: "success" });
    } catch (e) {
      throw new HTTPException(500, { message: "Internal Server Error" });
    }
  }
});

export { item };
