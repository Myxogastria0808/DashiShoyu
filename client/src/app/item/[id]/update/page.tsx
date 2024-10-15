import { Loading, UpdateItemForm } from "@/app";
import { Suspense } from "react";
import ky from "ky";
import { ItemDataType } from "@/types/userModel";

const UpdateItem = async ({ params }: { params: { id: string } }) => {
  const data: ItemDataType = await ky
    .get(`http://localhost:5000/api/item/get/${params.id}`)
    .json();
  let connector: { connector: string }[] = [];
  if (data.connector.length === 0) {
    connector = [{ connector: "" }];
  } else {
    connector = data.connector.map((connecotr_type) => ({
      connector: connecotr_type,
    }));
  }
  return (
    <Suspense fallback={<Loading />}>
      <UpdateItemForm item={data} connector={connector} />
    </Suspense>
  );
};

export default UpdateItem;
