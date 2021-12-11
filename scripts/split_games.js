#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

// CHANGEME
const DATASET = process.env.DATASET || "twic1413";

try {
  const data = fs.readFileSync(`./data/${DATASET}.pgn`, "utf8");
  data // Split on new line
    .split(/\n{2,}/g)
    // Remove blank lines
    .filter((p) => p.trim())
    .reduce((result, _, index, arr) => {
      if (index % 2 === 0) {
        result.push(arr.slice(index, index + 2));
      }
      return result;
    }, [])
    .forEach((game, index) => {
      fs.writeFile(path.join(process.cwd(), `tests/games/${DATASET}-${index}.pgn`), game.join("\n"), function (err) {
        if (err) return console.log(err);
      });
    });
} catch (err) {
  console.error(err);
}
