
const socket = new WebSocket("ws://127.0.0.1:4020", "battle-beetles");

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

socket.onmessage = (event) => {
  console.log("received")
  const data = JSON.parse(event.data);
  console.log(data)

  const beetle = data.beetles[0];
  const position = beetle.position;

  ctx.clearRect(0, 0, 300, 300);
  ctx.fillRect(position.x, position.y, 10, 10);

  for (let food of data.food) {
    ctx.fillRect(food.position.x, food.position.y, 5, 5);
  }
  
}

socket.onopen = (event) => {
  //console.log("sending yolo")
  //socket.send("Yolo")
}

socket.onclose = (event) => {
  console.log("Es closy");
}
