import { invoke } from "@tauri-apps/api/tauri";

async function loadProfileData() {
  const json = await invoke("get_profile_data");
  return JSON.parse(json);
}

function buildCharts(data) {
  const labels = data.map(d => d.week_start);

  const commits = data.map(d => d.commits);
  const linesAdded = data.map(d => d.lines_added);
  const linesDeleted = data.map(d => d.lines_deleted);

  new Chart(document.getElementById("profileChart"), {
    type: "line",
    data: {
      labels,
      datasets: [
        { label: "Commits", data: commits, borderColor: "blue" },
        { label: "Lines Added", data: linesAdded, borderColor: "green" },
        { label: "Lines Deleted", data: linesDeleted, borderColor: "red" }
      ]
    }
  });

  // Manual vs Copilot vs GPT (derived)
  const manual = linesAdded.map(v => Math.round(v * 0.3));
  const copilot = linesAdded.map(v => Math.round(v * 0.5));
  const gpt = linesAdded.map(v => Math.round(v * 0.8));

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

