(function () {
  var script = document.currentScript;
  var publicKey = script && script.getAttribute("data-perch-key");
  var gatewayUrl = script && script.getAttribute("data-perch-gateway");
  var sessionKey = "perch_session_id";
  var sessionId = readSessionId(sessionKey);

  if (!publicKey || document.querySelector("[data-perch-widget-root]")) {
    return;
  }

  if (!gatewayUrl) {
    gatewayUrl = new URL(script.src).origin;
  }

  if (!sessionId) {
    sessionId = createId();
    writeSessionId(sessionKey, sessionId);
  }

  var state = {
    open: false,
    connected: false,
    title: "Ask this site",
    status: "connecting",
    messages: [
      {
        role: "assistant",
        text: "Hi, I am Perch. Ask me about this site and I will answer with sources.",
        citations: []
      }
    ]
  };

  var host = document.createElement("div");
  host.setAttribute("data-perch-widget-root", "true");
  var shadow = host.attachShadow({ mode: "open" });
  document.body.appendChild(host);

  function render() {
    shadow.innerHTML = [
      "<style>",
      css(),
      "</style>",
      '<div class="perch-root" data-open="' + state.open + '">',
      state.open ? panel() : "",
      '<button class="perch-launcher" type="button" aria-label="Open Perch assistant">',
      '<span class="perch-mark">P</span>',
      "<span>Ask</span>",
      "</button>",
      "</div>"
    ].join("");

    shadow.querySelector(".perch-launcher").addEventListener("click", function () {
      state.open = !state.open;
      render();
      dispatch(state.open ? "perch:open" : "perch:close", {});
    });

    var close = shadow.querySelector("[data-perch-close]");
    if (close) {
      close.addEventListener("click", function () {
        state.open = false;
        render();
        dispatch("perch:close", {});
      });
    }

    var form = shadow.querySelector("form");
    if (form) {
      form.addEventListener("submit", function (event) {
        event.preventDefault();
        var input = shadow.querySelector("input");
        var question = input.value.trim();
        if (!question) {
          return;
        }
        input.value = "";
        ask(question);
      });
    }

    var messages = shadow.querySelector(".perch-messages");
    if (messages) {
      messages.scrollTop = messages.scrollHeight;
    }
  }

  function panel() {
    return [
      '<section class="perch-panel" aria-label="Perch assistant">',
      '<header class="perch-header">',
      '<span class="perch-mark">P</span>',
      "<div>",
      "<strong>" + escapeText(state.title) + "</strong>",
      "<small>" + escapeText(state.connected ? "Connected to Perch" : state.status) + "</small>",
      "</div>",
      '<button data-perch-close type="button" aria-label="Close Perch assistant">×</button>',
      "</header>",
      '<div class="perch-messages">',
      state.messages.map(message).join(""),
      state.status === "thinking" ? '<div class="perch-thinking"><span></span><span></span><span></span></div>' : "",
      "</div>",
      '<form class="perch-form">',
      '<input autocomplete="off" placeholder="Ask a question" aria-label="Ask a question" />',
      '<button type="submit">Send</button>',
      "</form>",
      "</section>"
    ].join("");
  }

  function message(item) {
    return [
      '<article class="perch-message" data-role="' + item.role + '">',
      "<p>" + escapeText(item.text) + "</p>",
      citations(item.citations || []),
      "</article>"
    ].join("");
  }

  function citations(items) {
    if (!items.length) {
      return "";
    }

    return [
      '<div class="perch-citations">',
      items.map(function (citation) {
        var url = safeUrl(citation.url);
        if (!url) {
          return "";
        }
        return '<a href="' + escapeAttribute(url) + '" target="_blank" rel="noreferrer">' + escapeText(citation.title || url) + "</a>";
      }).join(""),
      "</div>"
    ].join("");
  }

  function ask(question) {
    state.messages.push({ role: "visitor", text: question, citations: [] });
    state.status = "thinking";
    render();
    dispatch("perch:question", { question: question });

    fetch(gatewayUrl.replace(/\/$/, "") + "/v1/widget/chat", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        public_key: publicKey,
        session_id: sessionId,
        message: question
      })
    })
      .then(function (response) {
        if (!response.ok) {
          throw new Error("Perch request failed");
        }
        return response.json();
      })
      .then(function (answer) {
        state.messages.push({
          role: "assistant",
          text: answer.answer,
          citations: answer.citations || []
        });
        state.status = "ready";
        render();
        dispatch("perch:answer", { citations: answer.citations || [] });
      })
      .catch(function () {
        state.messages.push({
          role: "assistant",
          text: "Perch could not reach this site's indexed answers. Try again after the site is indexed.",
          citations: []
        });
        state.status = "error";
        render();
        dispatch("perch:error", {});
      });
  }

  function loadConfig() {
    fetch(gatewayUrl.replace(/\/$/, "") + "/v1/widget/config?key=" + encodeURIComponent(publicKey))
      .then(function (response) {
        if (!response.ok) {
          throw new Error("Perch config failed");
        }
        return response.json();
      })
      .then(function (config) {
        state.connected = true;
        state.status = "ready";
        state.title = "Ask " + config.site_name;
        render();
      })
      .catch(function () {
        state.connected = false;
        state.status = "demo fallback";
        render();
      });
  }

  function dispatch(name, detail) {
    window.dispatchEvent(new CustomEvent(name, { detail: detail }));
  }

  function createId() {
    if (window.crypto && window.crypto.randomUUID) {
      return window.crypto.randomUUID();
    }
    return "session_" + Math.random().toString(16).slice(2) + Date.now().toString(16);
  }

  function readSessionId(key) {
    try {
      return window.localStorage.getItem(key);
    } catch (error) {
      return "";
    }
  }

  function writeSessionId(key, value) {
    try {
      window.localStorage.setItem(key, value);
    } catch (error) {
      return;
    }
  }

  function safeUrl(value) {
    try {
      var url = new URL(value);
      if (url.protocol === "http:" || url.protocol === "https:") {
        return url.href;
      }
      return "";
    } catch (error) {
      return "";
    }
  }

  function escapeText(value) {
    return String(value || "")
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;");
  }

  function escapeAttribute(value) {
    return escapeText(value).replace(/"/g, "&quot;");
  }

  function css() {
    return [
      ":host{all:initial;color-scheme:light}",
      ".perch-root{position:fixed;right:22px;bottom:22px;z-index:2147483647;font-family:Inter,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;color:#14241d}",
      ".perch-launcher{height:48px;display:inline-flex;align-items:center;gap:9px;border:0;border-radius:999px;background:#12b76a;color:#fff;padding:0 17px;box-shadow:0 14px 28px -16px rgba(20,36,29,.65);font:700 14px/1 Inter,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;cursor:pointer}",
      ".perch-mark{width:28px;height:28px;display:inline-grid;place-items:center;border-radius:9px;background:#14241d;color:#fff;font-weight:800}",
      ".perch-panel{width:min(380px,calc(100vw - 28px));height:min(560px,calc(100vh - 112px));margin-bottom:12px;display:flex;flex-direction:column;overflow:hidden;border:1px solid #e7e4d8;border-radius:16px;background:#fcfbf7;box-shadow:0 30px 70px -28px rgba(20,36,29,.48)}",
      ".perch-header{display:flex;align-items:center;gap:11px;padding:14px;background:#14241d;color:#fff}",
      ".perch-header strong{display:block;font-size:14px}",
      ".perch-header small{display:block;margin-top:2px;color:#9cb3a8;font-size:12px}",
      ".perch-header button{margin-left:auto;width:32px;height:32px;border:1px solid rgba(255,255,255,.14);border-radius:999px;background:rgba(255,255,255,.08);color:#fff;font-size:22px;cursor:pointer}",
      ".perch-messages{flex:1;display:flex;flex-direction:column;gap:10px;overflow:auto;padding:14px;background:linear-gradient(180deg,#fcfbf7,#f6f4ec)}",
      ".perch-message{max-width:86%;border:1px solid #efede3;border-radius:13px;padding:10px 11px;background:#fff}",
      ".perch-message[data-role='visitor']{align-self:flex-end;border-color:#bee8d0;background:#eaf7ef}",
      ".perch-message p{margin:0;font-size:13px;line-height:1.45;color:#14241d}",
      ".perch-citations{display:flex;flex-wrap:wrap;gap:6px;margin-top:9px}",
      ".perch-citations a{max-width:100%;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;border:1px solid #c5ead5;border-radius:999px;background:#eaf7ef;color:#0e7a47;text-decoration:none;padding:4px 7px;font-size:11px;font-weight:700}",
      ".perch-thinking{display:flex;gap:5px;align-items:center;width:max-content;border:1px solid #efede3;border-radius:999px;background:#fff;padding:9px 11px}",
      ".perch-thinking span{width:6px;height:6px;border-radius:50%;background:#12b76a;animation:perch-bounce 900ms ease-in-out infinite}",
      ".perch-thinking span:nth-child(2){animation-delay:120ms}",
      ".perch-thinking span:nth-child(3){animation-delay:240ms}",
      "@keyframes perch-bounce{0%,80%,100%{opacity:.35;transform:translateY(0)}40%{opacity:1;transform:translateY(-4px)}}",
      ".perch-form{display:grid;grid-template-columns:1fr auto;gap:8px;padding:12px;border-top:1px solid #e7e4d8;background:#fff}",
      ".perch-form input{min-width:0;border:1px solid #d8d5c8;border-radius:999px;padding:0 12px;font:500 13px/38px Inter,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;color:#14241d}",
      ".perch-form button{height:38px;border:0;border-radius:999px;background:#14241d;color:#fff;padding:0 14px;font:700 13px/1 Inter,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;cursor:pointer}",
      "@media(max-width:520px){.perch-root{right:12px;bottom:12px;left:12px}.perch-panel{width:100%;height:min(560px,calc(100vh - 92px))}.perch-launcher{float:right}}"
    ].join("");
  }

  render();
  loadConfig();
})();
