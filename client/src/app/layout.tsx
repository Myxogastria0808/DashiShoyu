import type { Metadata } from "next";
import { Noto_Serif_JP } from "next/font/google";
import "./globals.css";

const font = Noto_Serif_JP({
  weight: ["400"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "DashiShoyu",
  description:
    "DashiShoyuは、筑波大学の学園祭実行委員会情報メディアシステム局が管理する物品を管理するシステムです。",
  icons: {
    icon: "/og-image.png",
    shortcut: "/og-image.png",
    apple: "/og-image.png",
    other: {
      rel: "apple-touch-icon-precomposed",
      url: "/og-image.png",
    },
  },
  openGraph: {
    title: "DashiShoyu",
    description:
      "DashiShoyuは、筑波大学の学園祭実行委員会情報メディアシステム局が管理する物品を管理するシステムです。",
    url: "https://github.com/sohosai/DashiShoyu",
    siteName: "DashiShoyu",
    type: "website",
    images: "/og-image.png",
    locale: "ja_JP",
  },
  twitter: {
    card: "summary_large_image",
    title: "DashiShoyu",
    description:
      "DashiShoyuは、筑波大学の学園祭実行委員会情報メディアシステム局が管理する物品を管理するシステムです。",
    images: {
      url: "/og-image.png",
      alt: "DashiShoyu",
    },
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={font.className}>{children}</body>
    </html>
  );
}
