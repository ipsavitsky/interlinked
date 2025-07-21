import initSync, { solution_wrapper } from './pkg/frontend.js';

async function run() {
  console.log("starting...");
  initSync();
  const response = await fetch("http://localhost:3001/difficulty");
  const difficulty = Number(await response.text());
  console.log("calculating solution");
  const result = solution_wrapper(difficulty, BigInt(Date.now()));
  console.log("solution calculated:", result);
}

run();
