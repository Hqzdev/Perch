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

export type WidgetChatResponse = {
  conversation_id: string;
  message_id: string;
  answer: string;
  citations: Array<{
    title: string;
    url: string;
  }>;
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

  async widgetChat(sessionId: string, message: string): Promise<WidgetChatResponse> {
    const response = await fetch(new URL("/v1/widget/chat", this.gatewayUrl), {
      method: "POST",
      headers: {
        accept: "application/json",
        "content-type": "application/json"
      },
      body: JSON.stringify({
        public_key: this.widgetKey,
        session_id: sessionId,
        message
      })
    });

    if (!response.ok) {
      throw new Error(`Gateway returned ${response.status}`);
    }

    return response.json();
  }
}
