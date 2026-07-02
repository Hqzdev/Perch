"use server";

import { revalidatePath } from "next/cache";

const gatewayUrl = process.env.NEXT_PUBLIC_PERCH_GATEWAY_URL ?? "http://localhost:18080";
const ownerToken = process.env.PERCH_OWNER_TOKEN ?? "perch_dev_owner_token";

export async function createSiteAction(formData: FormData) {
  const organizationName = stringField(formData, "organization_name");
  const siteName = stringField(formData, "site_name");
  const origin = stringField(formData, "origin");

  await ownerRequest("/v1/sites", {
    method: "POST",
    body: JSON.stringify({
      organization_name: organizationName,
      site_name: siteName,
      origin
    })
  });

  revalidatePath("/dashboard");
}

export async function indexPageAction(formData: FormData) {
  const siteId = stringField(formData, "site_id");
  const url = stringField(formData, "url");
  const title = optionalStringField(formData, "title");
  const content = stringField(formData, "content");

  await ownerRequest(`/v1/sites/${siteId}/pages`, {
    method: "POST",
    body: JSON.stringify({
      url,
      title,
      content,
      content_type: "text"
    })
  });

  revalidatePath("/dashboard");
}

export async function askTestQuestionAction(formData: FormData) {
  const publicKey = stringField(formData, "script_key");
  const origin = stringField(formData, "origin");
  const message = stringField(formData, "message");

  const response = await fetch(`${gatewayUrl}/v1/widget/chat`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      origin
    },
    body: JSON.stringify({
      public_key: publicKey,
      session_id: "dashboard-test",
      message
    })
  });

  if (!response.ok) {
    throw new Error(`Widget request failed with ${response.status}`);
  }

  revalidatePath("/dashboard");
}

async function ownerRequest(path: string, init: RequestInit) {
  const response = await fetch(`${gatewayUrl}${path}`, {
    ...init,
    headers: {
      "content-type": "application/json",
      "x-perch-owner-token": ownerToken,
      ...init.headers
    }
  });

  if (!response.ok) {
    throw new Error(`Gateway request failed with ${response.status}`);
  }
}

function stringField(formData: FormData, name: string) {
  const value = formData.get(name);

  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`${name} is required`);
  }

  return value.trim();
}

function optionalStringField(formData: FormData, name: string) {
  const value = formData.get(name);

  if (typeof value !== "string") {
    return null;
  }

  const trimmed = value.trim();

  return trimmed.length === 0 ? null : trimmed;
}
