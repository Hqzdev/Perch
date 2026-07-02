"use client";

import { useState } from "react";

type InstallSnippetButtonProps = {
  snippet: string;
};

export function InstallSnippetButton({ snippet }: InstallSnippetButtonProps) {
  const [label, setLabel] = useState("Copy");

  async function copySnippet() {
    try {
      if (navigator.clipboard?.writeText) {
        await navigator.clipboard.writeText(snippet);
      } else {
        copyWithField(snippet);
      }

      setLabel("Copied");
      window.setTimeout(() => setLabel("Copy"), 1400);
    } catch {
      if (copyWithField(snippet)) {
        setLabel("Copied");
      } else {
        setLabel("Failed");
      }

      window.setTimeout(() => setLabel("Copy"), 1400);
    }
  }

  return (
    <button className="dashboard-icon-button" type="button" onClick={copySnippet}>
      {label}
    </button>
  );
}

function copyWithField(value: string) {
  const field = document.createElement("textarea");
  field.value = value;
  field.setAttribute("readonly", "true");
  field.style.position = "fixed";
  field.style.left = "-9999px";
  field.style.top = "0";
  document.body.append(field);
  field.focus();
  field.select();
  const copied = document.execCommand("copy");
  field.remove();

  return copied;
}
