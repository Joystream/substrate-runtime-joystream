const fs = require("fs");
const path = require("path");

const componentsPath = path.resolve(__dirname, "../src/components");
const indexPath = path.resolve(__dirname, "../src");

let statements = fs
  .readdirSync(componentsPath)
  .map(file => {
    let filePath = path.resolve(componentsPath, file);
    let relPath = path.relative(indexPath, filePath);
    let { dir, name } = path.parse(relPath);
    return `export { default as ${name} } from \"./${dir}/${name}\";`;
  })
  .join("\n");

fs.writeFileSync(path.resolve(indexPath, "index.ts"), statements);
