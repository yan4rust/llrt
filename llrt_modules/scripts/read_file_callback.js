// will error with llrt_demo, no readFile function in FsModule

import * as fs from "node:fs";

console.log("--- read file with callback ---");

fs.readFile("scripts/test.txt","utf8",(err,data)=>{if(err){console.log(err);return}console.log(data)});
// fs.readFile("scripts/test.txt", "utf8", (err, data) => {
//     if (err) {
//         console.log(err);
//         return;
//     }
//     console.log(data);
// });


