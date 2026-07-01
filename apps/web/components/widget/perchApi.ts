export type WidgetConfig = {
  site_name: string;
  theme: {
    accent_color: string;
    placement: string;
  };
  features: {
    citations: boolean;
    streaming: boolean;
  };
};

export class PerchApiClient {
  private readonly gatewayUrl: string;
  private readonly widgetKey: string;

  constructor(gatewayUrl: string, widgetKey: string) {
    this.gatewayUrl = gatewayUrl.replace(/\/$/, "");
    this.widgetKey = widgetKey;
  }

  configured(): boolean {
    return this.gatewayUrl.length > 0 && this.widgetKey.length > 0;
  }

  async widgetConfig(): Promise<WidgetConfig> {
    const url = new URL("/v1/widget/config", this.gatewayUrl);
    url.searchParams.set("key", this.widgetKey);

    const response = await fetch(url, {
      headers: {
        accept: "application/json"
      }
    });

    if (!response.ok) {
      throw new Error(`Gateway returned ${response.status}`);
    }

    return response.json();
  }
}
