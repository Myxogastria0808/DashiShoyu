import { Suspense } from "react";
import { Loading, GetItem } from "@/app";
import { ItemDataType } from "@/types/userModel";
import ky from "ky";

const RegisterItem = async ({ params }: { params: { id: string } }) => {
  const data: ItemDataType = await ky
    .get(`http://localhost:5000/api/item/get/${params.id}`)
    .json();
  return (
    <>
      <Suspense fallback={<Loading />}>
        <GetItem item={data} />
      </Suspense>
    </>
  );
};

export default RegisterItem;
