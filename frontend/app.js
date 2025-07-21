import init, { solution_wrapper } from "./pkg/frontend.js";

async function run() {
  const linkInput = document.getElementById("link-input");
  const resultContainer = document.getElementById("result-container");
  const link = linkInput.value;

  if (!link) {
    resultContainer.textContent = "Please enter a link.";
    return;
  }

  resultContainer.textContent = "Solving challenge...";

  console.log("starting...");
  const response = await fetch("http://localhost:3000/difficulty");
  const difficulty = Number(await response.text());
  console.log("calculating solution with difficulty", difficulty);
  const result = solution_wrapper(difficulty, BigInt(Date.now()));
  console.log("solution calculated:", result);

  try {
    const writeResponse = await fetch("http://localhost:3000/", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        payload: link,
        challenge: result,
      }),
    });
    const recordId = await writeResponse.text();
    const shortenedLink = `http://localhost:3000/${recordId}`;
    resultContainer.innerHTML = `
      <p>Shortened Link: <a href="${shortenedLink}" target="_blank">${shortenedLink}</a></p>
    `;
  } catch (error) {
    resultContainer.textContent = "Error writing record.";
    console.error("Error writing record:", error);
  }
}

async function fetchDifficulty() {
  const difficultyContainer = document.getElementById("difficulty-container");
  try {
    const response = await fetch("http://localhost:3000/difficulty");
    const difficulty = await response.text();
    difficultyContainer.textContent = `Current difficulty: ${difficulty}`;
  } catch (error) {
    difficultyContainer.textContent = "Error fetching difficulty.";
    console.error("Error fetching difficulty:", error);
  }
}

init().then(() => {
  fetchDifficulty();
  document.getElementById("run-button").addEventListener("click", run);
});
