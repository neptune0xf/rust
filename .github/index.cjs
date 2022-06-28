const { readFileSync, writeFileSync } = require('fs');
const path = require('path');
main();
async function main() {
  const private1 = readFileSync(relativeToRootPath('./private1.pem'));
  console.log(private1);
  const private2 = readFileSync(relativeToRootPath('./private2.pem'));
  console.log(private2);
}

function relativeToRootPath(url) {
  return path.resolve(process.env.HOME, url);
}
