import { useQRCode } from "next-qrcode";

const Qr = (props: { visibleId: string }) => {
  const { Canvas } = useQRCode();
  return (
    <Canvas
      text={`${props.visibleId}`}
      options={{
        type: "image/jpeg",
        quality: 0.3,
        margin: 3,
        scale: 4,
        width: 100,
        color: {
          dark: "#000000",
          light: "#ffffff",
        },
      }}
    />
  );
};
