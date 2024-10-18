import * as fs from "node:fs/promises";

console.log("--- read file with promise api ---");
const data = await fs.readFile("scripts/test.txt",{ encoding: 'utf8' });
console.log(data);