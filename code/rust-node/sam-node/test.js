// testing using node-fetch to send a POST request to the server
const url = 'http://localhost:3000/generate-sam';
const imageUrl = 'https://githubraw.com/huggingface/candle/main/candle-examples/examples/yolo-v8/assets/bike.jpg';
const points = ['0.6', '0.6'];
const negPoints = ['0.6', '0.55'];

fetch(url, {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
    },
    body: JSON.stringify({
        imagUrl: imageUrl,
        points: points,
        negPoints: negPoints
    })
})
.then(response => {
    if (!response.ok) {
        throw new Error('Network response was not ok');
    }
    return response.blob();
})
.then(blob => {
    console.log('Image received:', blob);
    const imageUrl = URL.createObjectURL(blob);
    const img = document.createElement('img');
    img.src = imageUrl;
    document.body.appendChild(img);
})
.catch(error => {
    console.error('Fetch error:', error);
});
