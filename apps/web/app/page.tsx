import { readFileSync } from "node:fs";
import { join } from "node:path";

export default function Home() {
  const templateHtml = readFileSync(join(process.cwd(), "app/landing-template.html"), "utf8");

  return <div dangerouslySetInnerHTML={{ __html: templateHtml }} />;
}
