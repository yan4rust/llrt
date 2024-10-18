// url
let url = new URL("https://deno.land/manual/introduction");
url = new URL("/manual/introduction", "https://deno.land");
console.log(url.href); // https://deno.land/manual/introduction
console.log(url.host); // deno.land
console.log(url.origin); // https://deno.land
console.log(url.pathname); // /manual/introduction
console.log(url.protocol); // https:
url = new URL("https://docs.deno.com/api/deno/~/Deno.readFile");

console.log(url.searchParams.get("s")); // Deno.readFile
url.host = "deno.com";
url.protocol = "http:";

console.log(url.href); // http://deno.com/api?s=Deno.readFile