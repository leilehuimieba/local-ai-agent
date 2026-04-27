import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles/index.css";

const tokenMeta = document.querySelector('meta[name="local-agent-token"]') as HTMLMetaElement | null;
const token = tokenMeta?.content || "";

if (token) {
  const originalFetch = window.fetch;
  window.fetch = (input: RequestInfo | URL, init?: RequestInit) => {
    const url = typeof input === "string" ? input : input.toString();
    if (url.startsWith("/api/")) {
      const nextInit = init || {};
      const headers = new Headers(nextInit.headers);
      headers.set("X-Local-Agent-Token", token);
      nextInit.headers = headers;
      return originalFetch(input, nextInit);
    }
    return originalFetch(input, init);
  };
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
