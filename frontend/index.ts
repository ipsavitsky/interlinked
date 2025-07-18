import init, { solution_wrapper } from "frontend";

async function run() {
    await init();
    console.log(solution_wrapper(3));
}

run().catch(console.error);
