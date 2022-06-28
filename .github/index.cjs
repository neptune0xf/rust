const { readFileSync, writeFileSync } = require('fs');
const path = require('path');
main();
async function main() {
  console.log(process.env);
  const private1 = readFileSync(
    relativeToRootPath('./private1.pem')
  ).toString();
  console.log('private1', private1);
  const private2 = readFileSync(
    relativeToRootPath('./private2.pem')
  ).toString();
  console.log('private2', private2);
  const admin = readFileSync(
    relativeToRootPath('.config/dfx/identity/admin/identity.pem')
  ).toString();
  console.log('admin', admin);
}

function relativeToRootPath(url) {
  return path.resolve(process.env.HOME, url);
}
