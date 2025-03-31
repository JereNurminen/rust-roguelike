import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import api from "./api";

type WindowType = Window & {
  api: typeof api;
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <App />,
);

(window as unknown as WindowType).api = api;
