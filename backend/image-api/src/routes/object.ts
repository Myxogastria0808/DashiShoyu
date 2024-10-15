import { Hono } from "hono";
import { HTTPException } from "hono/http-exception";
import { deleteImage, uploadImage } from "../utils/r2";
import { getExtension } from "../utils/mimeTypes";

const object = new Hono();

object.post("/upload/:id", async (c) => {
  const id = c.req.param('id');
  const body: {
    mime_type: string;
    [x: string]: string | File;
  } = await c.req.parseBody();
  const mime_type: string = body["mime_type"];
  const extension: string = getExtension(mime_type);
  if (extension === "error") {
    throw new HTTPException(500, { message: "Internal Server Error: Invald mime type" });
  }
  const file: string | File = body["file"];
  if (typeof file === "string") {
    throw new HTTPException(401, { message: "Invald file type" });
  } else {
    try {
      let buffer: Buffer = Buffer.from(await file.arrayBuffer());
      await uploadImage(`obj-${id}.${extension}`, mime_type, buffer);
      console.log(`[INFO]: obj-${id}.${extension} was updated (date: ${new Date()})`);
      c.status(200);
      return c.json({ message: "File upload was successful." });
    } catch (e) {
      throw new HTTPException(500, { message: `Internal Server Error: ${e}` });
    }
  }
});

object.delete("/delete/:id", async (c) => {
  const id = c.req.param('id');
  const body: { mime_type: string; } = await c.req.parseBody();
  const mime_type: string = body["mime_type"];
  const extension: string = getExtension(mime_type);
  if (extension === "error") {
    throw new HTTPException(500, { message: "Internal Server Error: Invald mime type" });
  }
  try {
    await deleteImage(`obj-${id}.${extension}`);
    console.log(`[INFO]: obj-${id}.${extension} was deleted (date: ${new Date()})`);
    c.status(200);
  } catch (e) {
    throw new HTTPException(500, { message: `Internal Server Error: ${e}` });
  }
});

export { object };
