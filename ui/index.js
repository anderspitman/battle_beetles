
const socket = new WebSocket("ws://127.0.0.1:2794", "battle-beetles");

socket.onmessage = (event) => {
  console.log("received")
  console.log(event.data)
  console.log(event);
}

socket.onopen = (event) => {
  //console.log("sending yolo")
  //socket.send("Yolo")
}
