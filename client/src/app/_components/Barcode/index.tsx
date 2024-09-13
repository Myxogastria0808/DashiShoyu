import { ReactBarcode } from "react-jsbarcode";

const Barcode = (props: { visibleId: string }) => {
  return (
    <ReactBarcode
      value={props.visibleId}
      options={{
        format: "code128",
        height: 50,
        fontSize: 18,
        marginTop: 0,
        marginBottom: 0,
      }}
    />
  );
};
