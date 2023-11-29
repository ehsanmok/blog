const express = require('express');
const nativeModule = require('.');
const app = express();
const port = 3000;
const axios = require('axios');
const fs = require('fs');
const path = require('path');

async function downloadImage(url, filePath) {
  try {
    const response = await axios({
      method: 'GET',
      url: url,
      responseType: 'stream'
    });
    const writer = fs.createWriteStream(filePath);
    response.data.pipe(writer);

    return new Promise((resolve, reject) => {
      writer.on('finish', resolve);
      writer.on('error', reject);
    });
  } catch (error) {
    console.error('Error downloading the image:', error);
    throw error;
  }
}

app.use(express.json());
app.listen(port, () => console.log(`Listening on port ${port}`));
app.post("/generate-sam", async (req, res) => {
    try {
        const { imagUrl, points, negPoints } = req.body;
        const name = imagUrl.split('/').pop();
        const filePath = path.join(__dirname, name);
        await downloadImage(imagUrl, filePath);
        await nativeModule.generateSam(filePath, points, negPoints);
    } catch (error) {
        console.log(error);
        res.status(500).send(error);
    }
});
