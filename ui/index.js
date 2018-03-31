const canvas = document.getElementById('canvas');
const stopButton = document.getElementById('stop-button');
//const ctx = canvas.getContext('2d');

const params = {
  width: 300,
  height: 300,
  //type: Two.Types.webgl,
};
const two = new Two(params).appendTo(canvas);

const visualBeetles = [];
const visualFoods = [];
//const vectorLines = [];

const socket = new WebSocket("ws://127.0.0.1:4020", "battle-beetles");

socket.onmessage = (event) => {
  const data = JSON.parse(event.data);

  // convert beetles object to array
  const beetles = Object.entries(data.beetles).map((tuple) => tuple[1]);
  //console.log(beetles);
  const foods = data.food;
  //console.log(foods);

  matchArrays(beetles, visualBeetles, createBeetle);
  matchArrays(foods, visualFoods, createFood);

  for (let i = 0; i < beetles.length; i++) {
    const beetle = beetles[i];
    drawBeetle(beetle, i);
  }

  for (let i = 0; i < foods.length; i++) {
    const food = foods[i];
    drawFood(food, i);
  }

  two.update();
}

socket.onopen = (event) => {
  //console.log("sending yolo")
  //socket.send("Yolo")
}

socket.onclose = (event) => {
  console.log("Es closy");
}

stopButton.onclick = (event) => {
  socket.send(JSON.stringify({ message_type: 'terminate' }))
}

function matchArrays(model, vis, createNew) {
  if (vis.length < model.length) {
    for (let i = vis.length; i < model.length; i++) {
      if (vis[i]) {
        vis[i].visible = true;
      }
      else {
        vis.push(createNew());
      }
    }
  }
  else if (vis.length > model.length) {
    for (let i = model.length; i < vis.length; i++) {
      vis[i].visible = false;
    }
  }
}

function createBeetle() {

  const body = two.makeRectangle(0, 0, 20, 20);
  body.fill = 'green';

  const head = two.makeCircle(17, 0, 7);
  head.fill = 'black';

  const newBeetle = two.makeGroup(body, head);

  //const vectorLine = two.makeLine(0, 0, 0, 0);
  //vectorLines.push(vectorLine);

  return newBeetle;
}

function createFood() {
  const newFood = two.makeRectangle(0, 0, 10, 10);
  newFood.fill = 'Tomato';
  return newFood;
}

function drawBeetle(beetle, index) {
  //console.log(index, beetle.num_eaten);
  visualBeetle = visualBeetles[index];
  visualBeetle.translation.set(beetle.position.x, beetle.position.y);
  visualBeetle.rotation = beetle.angle;
  //const line = vectorLines[index];
  //const [anchor1, anchor2] = line.vertices;
  //anchor1.set(beetle.position.x, beetle.position.y);
  //anchor2.set(beetle.position.x + (beetle.direction.x * 50),
  //  beetle.position.y + (beetle.direction.y * 50));
}

function drawFood(food, index) {
  visualFood = visualFoods[index];
  visualFood.translation.set(food.position.x, food.position.y);
}
