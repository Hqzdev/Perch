import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Perch | Source-cited AI assistant for your website",
  description: "Add an AI assistant to your website in one line of script. Perch answers from your site content with cited sources.",
  openGraph: {
    title: "Perch | Source-cited AI assistant for your website",
    description: "A drop-in AI assistant that answers from your website's content with cited sources.",
    type: "website"
  }
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
