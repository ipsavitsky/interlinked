import init, { solution_wrapper } from './pkg/frontend.js';

async function run() {
  console.log("starting...");
  await init();
  const response = await fetch("http://localhost:3000/difficulty");
  const difficulty = Number(await response.text());
  console.log("calculating solution with difficulty", difficulty);
  const result = solution_wrapper(difficulty, BigInt(Date.now()));
  console.log("solution calculated:", result);
}

run();
