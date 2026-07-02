# Widget

The Perch widget should be framework-free and embeddable on ordinary websites.

## Install Snippet

```html
<script src="http://localhost:18080/widget/perch.js" data-perch-key="pk_dev_..." data-perch-gateway="http://localhost:18080"></script>
```

The local portfolio build serves the framework-free widget script from Gateway:

```txt
GET /widget/perch.js
```

The demo install page is:

```txt
http://localhost:3000/widget-demo?key=pk_dev_...
```

The widget configuration is resolved through:

```txt
GET /v1/widget/config?key=pk_live_...
Origin: https://customer-site.example
```

The browser origin must match the site origin registered in Gateway.

## Requirements

- no React dependency on the customer site
- isolated styles
- responsive panel
- accessible controls
- non-streaming cited answers
- visible citations
- error state
- loading state
- closed bubble state
- open chat state

## Browser Boundary

The widget can send:

- public widget key
- visitor question
- anonymous session ID
- current page URL
- origin

The widget must not send:

- trusted tenant ID
- trusted site ID
- dashboard tokens
- provider keys

## Rendering Rules

- Render model output as text.
- Render citation links only after URL validation.
- Do not inject arbitrary HTML from model output.
- Keep widget CSS scoped.
- Avoid global CSS resets.

## Events

Planned client events:

- `perch:open`
- `perch:close`
- `perch:question`
- `perch:answer`
- `perch:error`

Events are for analytics and debugging. They must not leak sensitive content to third parties by default.
