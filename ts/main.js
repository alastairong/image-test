"use strict";
const fs = require('fs').promises;
async function readFileAsDataUrl(filePath) {
    const data = await fs.readFile(filePath);
    let base64Data = data.toString('base64');
    console.log(base64Data);
}
readFileAsDataUrl('../file');
