import { DashboardMockup } from "@/components/product/DashboardMockup";

export function DashboardPreviewSection() {
  return (
    <section className="section" id="dashboard">
      <div className="page-wrap">
        <p className="section-kicker">Dashboard</p>
        <h2 className="section-title">Control the crawl, widget, and question loop.</h2>
        <p className="section-copy">
          The dashboard is intentionally practical: indexing status, install snippet, visitor questions, theme settings, and source coverage.
        </p>
        <div style={{ marginTop: "38px" }}>
          <DashboardMockup />
        </div>
      </div>
    </section>
  );
}
