import { z } from "zod";

//時間の取得
const today = new Date();
//NaNをnumberとして扱えるようにする
const NaNSchema: z.ZodSchema<number> = z.any().refine(Number.isNaN);

const updateItemFormSchema = z.object({
  parent_visible_id: z
    .string()
    .min(4, { message: "物品IDは、4文字の英数字です。" })
    .max(4, { message: "物品IDは、4文字の英数字です。" })
    .regex(/^[A-Z0-9]+$/, {
      message: "半角英数字（英字は大文字）で入力してください",
    }),
  visible_id: z
    .string()
    .min(4, { message: "物品IDは、4文字の英数字です。" })
    .max(4, { message: "物品IDは、4文字の英数字です。" })
    .regex(/^[A-Z0-9]+$/, {
      message: "半角英数字（英字は大文字）で入力してください",
    }),
  color: z.enum(
    [
      "Red",
      "Orange",
      "Brown",
      "SkyBlue",
      "Blue",
      "Green",
      "Yellow",
      "Purple",
      "Pink",
    ],
    { message: "不正な値が入力されました。" }
  ),
  record: z.enum(["Qr", "Barcode", "Nothing"], {
    message: "不正な値が入力されました。",
  }),
  name: z
    .string()
    .refine((name) => name.length >= 1, { message: "必須項目です。" }),
  product_number: z.string().nullable(),
  description: z.string().nullable(),
  year_purchased: z
    .number()
    .or(NaNSchema)
    .refine(
      (year) =>
        Number.isNaN(year) ||
        (year >= 2000 && year <= today.getFullYear() && Number.isInteger(year)),
      {
        message: `西暦 2000年から、西暦 ${today.getFullYear()}年までの西暦の数字4桁を入力してください。`,
      }
    ),
  connector: z.custom<{ connector: string }[]>(),
  file: z.optional(
    z.custom<File>().refine((file) => file.size <= 2097152, {
      message: "ファイルサイズは最大20MBです",
    })
  ),
});

const registerItemFormSchema = z.object({
  parent_visible_id: z
    .string()
    .min(4, { message: "物品IDは、4文字の英数字です。" })
    .max(4, { message: "物品IDは、4文字の英数字です。" })
    .regex(/^[A-Z0-9]+$/, {
      message: "半角英数字（英字は大文字）で入力してください",
    }),
  visible_id: z
    .string()
    .min(4, { message: "物品IDは、4文字の英数字です。" })
    .max(4, { message: "物品IDは、4文字の英数字です。" })
    .regex(/^[A-Z0-9]+$/, {
      message: "半角英数字（英字は大文字）で入力してください",
    }),
  color: z.enum(
    [
      "Red",
      "Orange",
      "Brown",
      "SkyBlue",
      "Blue",
      "Green",
      "Yellow",
      "Purple",
      "Pink",
    ],
    { message: "不正な値が入力されました。" }
  ),
  record: z.enum(["Qr", "Barcode", "Nothing"], {
    message: "不正な値が入力されました。",
  }),
  name: z
    .string()
    .refine((name) => name.length >= 1, { message: "必須項目です。" }),
  product_number: z.string().nullable(),
  description: z.string().nullable(),
  year_purchased: z
    .number()
    .or(NaNSchema)
    .refine(
      (year) =>
        Number.isNaN(year) ||
        (year >= 2000 && year <= today.getFullYear() && Number.isInteger(year)),
      {
        message: `西暦 2000年から、西暦 ${today.getFullYear()}年までの西暦の数字4桁を入力してください。`,
      }
    ),
  connector: z.custom<{ connector: string }[]>(),
  file: z.custom<File>().refine((file) => file.size <= 2097152, {
    message: "ファイルサイズは最大20MBです",
  }),
});

export { updateItemFormSchema, registerItemFormSchema };
export type UpdateItemFormSchemaType = z.infer<typeof updateItemFormSchema>;
export type RegisterItemFormSchemaType = z.infer<typeof registerItemFormSchema>;
