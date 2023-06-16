const fs = require('fs').promises;

async function readFileAsDataUrl(filePath: string) {
    const data = await fs.readFile(filePath);
    let base64Data = data.toString('base64');
    console.log(base64Data);
  }
  
  readFileAsDataUrl('../file');