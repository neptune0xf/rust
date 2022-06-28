const { readFileSync, writeFileSync } = require('fs');
const path = require('path');
main();
async function main() {
  console.log(process.env);
  let key = readFileSync(
    relativeToRootPath('.config/dfx/identity/admin/identity.pem')
  ).toString();
  key = key.split('#').join('');
  console.log('key', key);
  writeFileSync(
    relativeToRootPath('.config/dfx/identity/admin/identity.pem'),
    key
  );
}

function relativeToRootPath(url) {
  return path.resolve(process.env.HOME, url);
}
