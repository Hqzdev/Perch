import Script from "next/script";

type WidgetDemoPageProps = {
  searchParams: Promise<{
    key?: string;
    gateway?: string;
  }>;
};

export default async function WidgetDemoPage({ searchParams }: WidgetDemoPageProps) {
  const params = await searchParams;
  const widgetKey = params.key ?? process.env.NEXT_PUBLIC_PERCH_WIDGET_KEY ?? "";
  const gatewayUrl = params.gateway ?? process.env.NEXT_PUBLIC_PERCH_GATEWAY_URL ?? "http://localhost:18080";

  return (
    <main className="widget-demo-shell">
      <section className="widget-demo-content">
        <p className="dashboard-eyebrow">Embeddable widget demo</p>
        <h1>Perch installed from a script tag</h1>
        <p>
          This page does not render the React demo widget. It loads the standalone framework-free
          widget from Gateway using the same script shape shown in the dashboard.
        </p>
        <pre>{`<script src="${gatewayUrl}/widget/perch.js" data-perch-key="${widgetKey}" data-perch-gateway="${gatewayUrl}"></script>`}</pre>
      </section>
      {widgetKey ? (
        <Script
          data-perch-gateway={gatewayUrl}
          data-perch-key={widgetKey}
          src={`${gatewayUrl}/widget/perch.js`}
          strategy="afterInteractive"
        />
      ) : null}
    </main>
  );
}
