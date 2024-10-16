/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  images: {
    domains: ["127.0.0.1:5000", "pub-236a41428aff4b1abfe53652a32e2b13.r2.dev"],
  },
};

export default nextConfig;
