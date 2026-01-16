import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ToastProvider, ToastContainer } from "./components/Toast";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ToastProvider>
      <App />
      <ToastContainer />
    </ToastProvider>
  </React.StrictMode>,
);
