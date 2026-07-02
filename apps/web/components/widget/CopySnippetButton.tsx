"use client";

import { useEffect, useState } from "react";
import { createPortal } from "react-dom";

const snippet = "<script src=\"https://cdn.perch.ai/widget.js\" data-perch-key=\"pk_live_8f2c...a91\"></script>";

async function copySnippetText() {
  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(snippet);
      return true;
    } catch {
      return copyWithField();
    }
  }

  return copyWithField();
}

function copyWithField() {
  const field = document.createElement("textarea");
  field.value = snippet;
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

export function CopySnippetButton() {
  const [target, setTarget] = useState<HTMLElement | null>(null);
  const [label, setLabel] = useState("copy");

  useEffect(() => {
    setTarget(document.getElementById("perch-copy-slot"));
  }, []);

  function copySnippet() {
    setLabel("copied");
    window.setTimeout(() => setLabel("copy"), 1400);
    void copySnippetText();
  }

  if (!target) {
    return null;
  }

  return createPortal(
    <button
      className="perch-copy-button"
      type="button"
      onClick={copySnippet}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          copySnippet();
        }
      }}
    >
      {label}
    </button>,
    target
  );
}
