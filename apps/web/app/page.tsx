import { readFileSync } from "node:fs";
import { join } from "node:path";
import { CopySnippetButton } from "@/components/widget/CopySnippetButton";
import { PerchWidget } from "@/components/widget/PerchWidget";
import { SiteInteractions } from "@/components/widget/SiteInteractions";

export default function Home() {
  const templateHtml = readFileSync(join(process.cwd(), "app/landing-template.html"), "utf8");

  return (
    <>
      <div dangerouslySetInnerHTML={{ __html: templateHtml }} />
      <CopySnippetButton />
      <SiteInteractions />
      <PerchWidget />
    </>
  );
}
