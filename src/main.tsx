import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import App from "./App";
import { UpdateWindow } from "./components/UpdateWindow/UpdateWindow";

const currentWindow = getCurrentWebviewWindow();
const windowLabel = currentWindow.label;

const renderApp = () => {
  if (windowLabel === "updater") {
    return <UpdateWindow />;
  }
  return <App />;
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {renderApp()}
  </React.StrictMode>,
);
