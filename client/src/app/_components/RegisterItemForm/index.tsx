"use client";
import {
  Controller,
  SubmitHandler,
  useForm,
  useFieldArray,
} from "react-hook-form";
import ky from "ky";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  registerItemFormSchema,
  RegisterItemFormSchemaType,
} from "@/types/itemValidation";
import { ItemDataType } from "@/types/userModel";

const RegisterItemForm = () => {
  const {
    register,
    handleSubmit,
    formState: { errors },
    control,
  } = useForm<RegisterItemFormSchemaType>({
    resolver: zodResolver(registerItemFormSchema),
    defaultValues: {
      color: "Red",
      record: "Qr",
    },
  });
  const onSubmit: SubmitHandler<RegisterItemFormSchemaType> = async (data) => {
    //空の文字列を削除
    const result_connector: { connector: string }[] = [];
    data.connector.map((connector, _index) => {
      if (connector.connector !== "") {
        result_connector.push({ connector: connector.connector });
      }
    });
    data.connector = result_connector;
    if (data.product_number === null) {
      data.product_number = "";
    }
    if (data.description === null) {
      data.description = "";
    }
    let result_year_purchased: string = "";
    if (Number.isNaN(data.year_purchased)) {
      result_year_purchased = "";
    } else {
      result_year_purchased = String(data.year_purchased);
    }
    console.log(data);
    console.log(result_year_purchased);
    //ファイル以外の更新データを送る
    const formData = new FormData();
    formData.append("parent_visible_id", data.parent_visible_id);
    formData.append("visible_id", data.visible_id);
    formData.append("color", data.color);
    formData.append("record", data.record);
    formData.append("name", data.name);
    formData.append("product_number", data.product_number);
    formData.append("description", data.description);
    formData.append("year_purchased", result_year_purchased);
    data.connector.forEach((connector, _index) => {
      formData.append("connector", connector.connector);
    });
    //物品の登録
    try {
      const url: string = `http://localhost:5000/api/item/register`;
      const register_item: ItemDataType = await ky
        .post(url, {
          body: formData,
        })
        .json();
      //画像のアップロード
      const imageFormData: FormData = new FormData();
      imageFormData.append("file", data.file);
      console.log(register_item);
      try {
        const image_url: string = `http://localhost:7000/api/item/upload/${register_item.id}`;
        await ky.post(image_url, { body: imageFormData });
      } catch (error) {
        throw new Error(`画像のアップロードに失敗しました。\n${error}`);
      }
    } catch (error) {
      throw new Error(`物品情報の登録に失敗しました。\n${error}`);
    }
  };
  const { fields, append, remove } = useFieldArray({
    name: "connector",
    control,
  });
  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <label htmlFor="parent_visible_id">Parent Visible Id: </label>
      <input id="parent_visible_id" {...register("parent_visible_id")} />
      <br />
      <p>{errors.parent_visible_id && errors.parent_visible_id.message}</p>
      <br />
      <label htmlFor="visible_id">Visible Id: </label>
      <input id="visible_id" {...register("visible_id")} />
      <br />
      <p>{errors.visible_id && errors.visible_id.message}</p>
      <br />
      <label htmlFor="color">Color: </label>
      <select id="color" {...register("color")}>
        <option value="Red">赤</option>
        <option value="Orange">橙</option>
        <option value="Brown">茶</option>
        <option value="SkyBlue">水</option>
        <option value="Blue">青</option>
        <option value="Green">緑</option>
        <option value="Yellow">黄</option>
        <option value="Purple">紫</option>
        <option value="Pink">桃</option>
      </select>
      <br />
      <p>{errors.color && errors.color.message}</p>
      <br />
      <label htmlFor="record">Record: </label>
      <select id="record" {...register("record")}>
        <option value="Qr">QR</option>
        <option value="Barcode">バーコード</option>
        <option value="Nothing">なし</option>
      </select>
      <br />
      <p>{errors.record && errors.record.message}</p>
      <br />
      <label htmlFor="name">Name: </label>
      <input id="name" {...register("name")} />
      <br />
      <p>{errors.name && errors.name.message}</p>
      <br />
      <label htmlFor="product_number">Product Number: </label>
      <input id="product_number" {...register("product_number")} />
      <br />
      <p>{errors.product_number && errors.product_number.message}</p>
      <br />
      <label htmlFor="description">Description: </label>
      <input id="description" {...register("description")} />
      <br />
      <p>{errors.description && errors.description.message}</p>
      <br />
      <label htmlFor="year_purchased">Year Purchased: </label>
      <input
        id="year_purchased"
        type="number"
        {...register("year_purchased", { valueAsNumber: true })}
      />
      <br />
      <p>{errors.year_purchased && errors.year_purchased.message}</p>
      <br />
      <label htmlFor="connector">Connector: </label>
      {fields.map((field, index) => (
        <div key={field.id}>
          <input id="connector" {...register(`connector.${index}.connector`)} />
          {index >= 0 && (
            <input type="submit" value="✕" onClick={() => remove(index)} />
          )}
        </div>
      ))}
      <br />
      <p>{errors.connector && errors.connector.message}</p>
      <br />
      <input
        type="button"
        value="端子の追加"
        onClick={() => append({ connector: "" })}
      />
      <br />
      <Controller
        control={control}
        name="file"
        render={({ field: { onChange } }) => (
          <>
            <label htmlFor="file">File: </label>
            <input
              id="file"
              type="file"
              accept="image/*"
              onChange={(e) => onChange(e.target.files?.[0])}
            />
          </>
        )}
      />
      <br />
      <p>{errors.file && errors.file.message}</p>
      <input type="submit" value="登録" />
    </form>
  );
};

export default RegisterItemForm;
