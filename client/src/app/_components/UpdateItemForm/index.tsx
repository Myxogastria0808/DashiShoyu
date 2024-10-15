"use client";
import {
  Controller,
  SubmitHandler,
  useForm,
  useFieldArray,
} from "react-hook-form";
import { ItemDataType } from "@/types/userModel";
import ky from "ky";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  updateItemFormSchema,
  UpdateItemFormSchemaType,
} from "@/types/itemValidation";

const UpdateItemForm = (props: {
  item: ItemDataType;
  connector: { connector: string }[];
}) => {
  let assertion_year_purchased: number | undefined;
  if (props.item.year_purchased === null) {
    assertion_year_purchased = undefined;
  } else {
    assertion_year_purchased = Number(props.item.year_purchased);
  }
  const {
    register,
    handleSubmit,
    formState: { errors },
    control,
  } = useForm<UpdateItemFormSchemaType>({
    resolver: zodResolver(updateItemFormSchema),
    defaultValues: {
      parent_visible_id: props.item.parent_visible_id,
      visible_id: props.item.visible_id,
      record: props.item.record as "Qr" | "Barcode" | "Nothing" | undefined,
      name: props.item.name,
      product_number: props.item.product_number,
      description: props.item.description,
      year_purchased: assertion_year_purchased,
      connector: props.connector,
    },
  });
  const onSubmit: SubmitHandler<UpdateItemFormSchemaType> = async (data) => {
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
    //ファイル以外の更新データを送る
    const formData: FormData = new FormData();
    formData.append("parent_visible_id", data.parent_visible_id);
    formData.append("visible_id", data.visible_id);
    formData.append("record", data.record);
    formData.append("name", data.name);
    formData.append("product_number", data.product_number);
    formData.append("description", data.description);
    formData.append("year_purchased", result_year_purchased);
    data.connector.forEach((connector, _index) => {
      formData.append("connector", connector.connector);
    });
    const url: string = `http://localhost:5000/api/item/update/${props.item.id}`;
    try {
      let result = await ky.put(url, { body: formData });
      console.log(result);
    } catch (error) {
      throw new Error(`物品情報の更新に失敗しました。\n${error}`);
    }
    //ファイルが存在する時だけ送る
    if (typeof data.file !== "undefined") {
      const imageFormData: FormData = new FormData();
      imageFormData.append("file", data.file);
      try {
        const image_url: string = `http://localhost:7000/api/item/upload/${props.item.id}`;
        await ky.post(image_url, { body: imageFormData });
      } catch (error) {
        throw new Error(`画像のアップロードに失敗しました。\n${error}`);
      }
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
        type="year_purchased"
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

export default UpdateItemForm;
