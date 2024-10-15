/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  images: {
    domains: ["127.0.0.1:5000", "pub-05386dd671a54d04958b914542c641bb.r2.dev"],
  },
};

export default nextConfig;
