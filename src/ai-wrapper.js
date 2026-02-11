import { invoke } from "@tauri-apps/api/core";

export async function trackedOpenAI(prompt) {

  const response = await fetch("https://api.openai.com/v1/responses", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": "Bearer YOUR_KEY"
    },
    body: JSON.stringify({
      model: "gpt-4.1-mini",
      input: prompt
    })
  });

  const data = await response.json();

  const tokensIn = data.usage?.input_tokens || 0;
  const tokensOut = data.usage?.output_tokens || 0;

  const cost = (tokensIn * 0.000005) + (tokensOut * 0.000015);

  await invoke("log_session", {
    provider: "openai",
    tokensIn,
    tokensOut,
    cost
  });

  return data;
}
