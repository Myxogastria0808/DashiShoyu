import {
  DeleteObjectCommand,
  GetObjectCommand,
  GetObjectCommandOutput,
  PutObjectCommand,
  S3Client,
} from "@aws-sdk/client-s3";
import dotenv from "dotenv";
import { DotEnvCaster } from "dotenv-caster";
import { Readable } from "stream";
import * as fs from "fs";

dotenv.config();

const dotenvCaster = new DotEnvCaster();
const endpoint: string = dotenvCaster.castString(
  process.env.CLOUDFLARE_URI_ENDPOINT
);
const accessKeyId: string = dotenvCaster.castString(
  process.env.API_TOKENS_ACCESS_KEY_ID
);
const secretAccessKey: string = dotenvCaster.castString(
  process.env.API_TOKENS_SECRET_ACCESS_KEY
);
const bucket: string = dotenvCaster.castString(process.env.BUCKET_NAME);

//r2のインスタンスを作成
const s3: S3Client = new S3Client({
  region: "apac",
  endpoint: endpoint,
  credentials: {
    accessKeyId: accessKeyId,
    secretAccessKey: secretAccessKey,
    // @ts-ignore
    signatureVersion: "v4",
  },
});

const uploadImage = async (
  file_name: string,
  content_type: string,
  buffer: Buffer
): Promise<void> => {
  await s3.send(
    new PutObjectCommand({
      Bucket: bucket,
      Key: file_name,
      ContentType: content_type,
      Body: buffer,
    })
  );
};

const deleteImage = async (
  file_name: string
): Promise<void> => {
  await s3.send(
    new DeleteObjectCommand({
      Bucket: bucket,
      Key: file_name,
    })
  )
};

const getImage = async (file_name: string): Promise<fs.WriteStream> => {
  const result: GetObjectCommandOutput = await s3.send(
    new GetObjectCommand({
      Bucket: bucket,
      Key: file_name,
    })
  );
  const readableObj: Readable = result.Body as Readable;
  const writableObj: fs.WriteStream = fs.createWriteStream(file_name);
  readableObj.pipe(writableObj);
  return writableObj;
};

export { s3, uploadImage, deleteImage, getImage };
