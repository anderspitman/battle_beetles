const canvas = document.getElementById('canvas');
//const ctx = canvas.getContext('2d');

const params = {
  width: 300,
  height: 300,
  //type: Two.Types.webgl,
};
const two = new Two(params).appendTo(canvas);

const visualBeetles = [];
const visualFoods = [];
const vectorLines = [];

const socket = new WebSocket("ws://127.0.0.1:4020", "battle-beetles");

socket.onmessage = (event) => {
  const data = JSON.parse(event.data);
  const beetles = data.beetles;
  const foods = data.food;

  matchArrays(beetles, visualBeetles, createBeetle);
  matchArrays(foods, visualFoods, createFood);

  for (let i = 0; i < data.beetles.length; i++) {
    const beetle = data.beetles[i];
    drawBeetle(beetle, i);
  }

  for (let i = 0; i < data.food.length; i++) {
    const food = data.food[i];
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

  const newBeetle = two.makeRectangle(0, 0, 40, 20);
  newBeetle.fill = 'SteelBlue';

  const vectorLine = two.makeLine(0, 0, 0, 0);
  vectorLines.push(vectorLine);

  return newBeetle;
}

function createFood() {
  const newFood = two.makeRectangle(0, 0, 10, 10);
  newFood.fill = 'Tomato';
  return newFood;
}

function drawBeetle(beetle, index) {
  if (index === 0) {
    console.log(beetle.angle);
  }
  visualBeetle = visualBeetles[index];
  visualBeetle.translation.set(beetle.position.x, beetle.position.y);
  visualBeetle.rotation = beetle.angle;
  const line = vectorLines[index];
  const [anchor1, anchor2] = line.vertices;
  anchor1.set(beetle.position.x, beetle.position.y);
  anchor2.set(beetle.position.x + (beetle.direction.x * 50),
    beetle.position.y + (beetle.direction.y * 50));
}

function drawFood(food, index) {
  visualFood = visualFoods[index];
  visualFood.translation.set(food.position.x, food.position.y);
}
