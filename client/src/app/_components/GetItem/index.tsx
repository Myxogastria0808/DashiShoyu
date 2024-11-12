import Image from "next/image";
import { ItemDataType } from "@/types/userModel";

const GetItem = (props: { item: ItemDataType }) => {
  return (
    <>
      <p>Id: {props.item.id}</p>
      <p>VisibleId: {props.item.visible_id}</p>
      <p>Name: {props.item.name}</p>
      <p>ProductNumber: {props.item.product_number}</p>
      <Image
        src={props.item.photo_url}
        width={0}
        height={0}
        sizes="100%"
        style={{ width: "500px", height: "auto" }}
        alt={"sample image"}
      />
      <p>Record: {props.item.record}</p>
      <p>Color: {props.item.color}</p>
      <p>Description: {props.item.description}</p>
      <p>YearPaurchased: {props.item.year_purchased}</p>
      <p>Connector: {props.item.connector}</p>
      <p>CreatedAt: {props.item.created_at}</p>
      <p>UpdatedAt: {props.item.updated_at}</p>
    </>
  );
};

export default GetItem;
