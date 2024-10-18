// read file with sync api 

// deno compatible module name
import * as fs from "node:fs";
console.log("--- read file with sync api ---");
try {
    const data = fs.readFileSync("scripts/test.txt", "utf8");
    console.log(data);
} catch (err) {
    console.log(err);
}