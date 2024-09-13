"use client";
import Image from "next/image";
import { ItemDataType } from "@/types/userModel";

const SearchItemResults = (props: { data: ItemDataType[] }) => {
  return (
    <ul>
      {props.data.map((item, index) => {
        return (
          <li key={index}>
            <p>Id: {item.id}</p>
            <p>VisibleId: {item.visible_id}</p>
            <p>ParentId: {item.parent_id}</p>
            <p>ParentVisibleId: {item.parent_visible_id}</p>
            <p>GrandParentId: {item.grand_parent_id}</p>
            <p>GrandParentVisibleId: {item.grand_parent_visible_id}</p>
            <p>Name: {item.name}</p>
            <p>ProductNumber: {item.product_number}</p>
            <Image
              src={item.photo_url}
              width={0}
              height={0}
              sizes="100%"
              style={{ width: "500px", height: "auto" }}
              alt={"sample image"}
            />
            <p>Record: {item.record}</p>
            <p>Color: {item.color}</p>
            <p>Description: {item.description}</p>
            <p>YearPaurchased: {item.year_purchased}</p>
            <p>Connector: {item.connector}</p>
            <p>CreatedAt: {item.created_at}</p>
            <p>UpdatedAt: {item.updated_at}</p>
          </li>
        );
      })}
    </ul>
  );
};

export default SearchItemResults;
