
const socket = new WebSocket("ws://127.0.0.1:4020", "battle-beetles");

socket.onmessage = (event) => {
  console.log("received")
  const data = JSON.parse(event.data);
  console.log(data)
}

socket.onopen = (event) => {
  //console.log("sending yolo")
  //socket.send("Yolo")
}

socket.onclose = (event) => {
  console.log("Es closy");
}
