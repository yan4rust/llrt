import * as fs from "node:fs/promises";

const data = await fs.readFile("scripts/test.txt",{ encoding: 'utf8' });
console.log(data);