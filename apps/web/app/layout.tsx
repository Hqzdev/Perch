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
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="" />
        <link
          href="https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,500;12..96,600;12..96,700;12..96,800&family=IBM+Plex+Sans:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500;600&display=swap"
          rel="stylesheet"
        />
      </head>
      <body>{children}</body>
    </html>
  );
}
