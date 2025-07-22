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
  const url = window.BACKEND_URL;
  const response = await fetch(`${url}/difficulty`);
  const difficulty = Number(await response.text());
  console.log("calculating solution with difficulty", difficulty);
  const result = solution_wrapper(difficulty, BigInt(Date.now()));
  console.log("solution calculated:", result);

  try {
    const writeResponse = await fetch(`${url}/`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        payload: link,
        challenge: result,
      }),
    });
    if (writeResponse.status != 200) {
      console.error(
        "Error writing record: returned code",
        writeResponse.status,
      );
      resultContainer.textContent =
        "Error writing record. " + (await writeResponse.text());
      return;
    }
    const recordId = await writeResponse.text();
    const shortenedLink = `${url}/${recordId}`;
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
    const url = window.BACKEND_URL;
    const response = await fetch(`${url}/difficulty`);
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
