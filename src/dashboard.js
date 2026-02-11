import { invoke } from "@tauri-apps/api/core";

async function calculateEconomics() {
  const sessions = parseInt(document.getElementById("sessions").value);
  const cost = parseFloat(document.getElementById("cost").value);
  const commits = parseInt(document.getElementById("commits").value);
  const hourlyRate = parseFloat(document.getElementById("hourlyRate").value);

  const report = await invoke("run_economic_model", {
    sessions,
    totalCost: cost,
    commits,
    hourlyRate
  });

  document.getElementById("valueProduced").innerText =
    "$" + report.value_produced.toFixed(2);

  document.getElementById("roi").innerText =
    report.roi.toFixed(2) + "x";

  document.getElementById("multiplier").innerText =
    report.productivity_multiplier.toFixed(2) + "x";
}

  new Chart(document.getElementById("gptChart"), {
    type: "line",
    data: {
      labels,
      datasets: [
        { label: "Manual", data: manual, borderColor: "black" },
        { label: "Copilot", data: copilot, borderColor: "blue" },
        { label: "GPT", data: gpt, borderColor: "purple" }
      ]
    }
  });
}

loadProfileData().then(buildCharts);

