const canvas = document.getElementById('canvas');
const stopButton = document.getElementById('stop-button');

const viewportDimensions = getViewportDimensions();
const buttonRowHeight = 50;

const params = {
  width: viewportDimensions.width,
  height: viewportDimensions.height - buttonRowHeight,
  //type: Two.Types.webgl,
};
const two = new Two(params).appendTo(canvas);

let shiftKeyDown = false;
window.onkeyup = function(e) {
  shiftKeyDown = false;
};
window.onkeydown = function(e) {
  if (e.key == 'Shift') {
    shiftKeyDown = true;
  }
};

const visualBeetles = [];
const visualFoods = [];
//const vectorLines = [];

const messageService = new messageServiceModule.MessageService();
const socket = messageService.getSocket();

drawBackground();

let messageCount = 0;
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

    if (visualBeetles[i].beetle._renderer && visualBeetles[i].beetle._renderer.elem) {
      visualBeetles[i].beetle._renderer.elem.onclick = (e) => {
        console.log(beetle)
        if (!shiftKeyDown) {
          messageService.deselectAllBeetles()
        }
        messageService.selectBeetle({ beetleId: beetle.id })
      };

      visualBeetles[i].beetle._renderer.elem.oncontextmenu = (e) => {
        e.preventDefault();
        messageService.selectedInteractCommand({ beetleId: beetle.id })
      };
    }

    drawBeetle(beetle, i);
  }

  for (let i = 0; i < foods.length; i++) {
    const food = foods[i];
    drawFood(food, i);
  }

  requestAnimationFrame(() => {
    two.update();
  });
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
        vis[i].beetle.visible = true;
      }
      else {
        vis.push(createNew());
      }
    }
  }
  else if (vis.length > model.length) {
    for (let i = model.length; i < vis.length; i++) {
      vis[i].beetle.visible = false;
        vis[i].selectedIndicator.visible = false;
    }
  }
}

function drawBackground() {
  const rect = two.makeRectangle(
    params.width / 2, params.height / 2, params.width, params.height);
  rect.fill = '#c3c3c3';

  two.update();

  rect._renderer.elem.onclick = (e) => {
    messageService.deselectAllBeetles();
  };

  rect._renderer.elem.oncontextmenu = (e) => {
    e.preventDefault();
    messageService.selectedMoveCommand({ x: e.clientX, y: e.clientY });
  };
}

function createBeetle() {

  const selectedIndicator = two.makeRectangle(0, 0, 50, 50);
  selectedIndicator.stroke = 'lightgreen';
  selectedIndicator.fill = 'none';

  const body = two.makeRectangle(0, 0, 20, 20);
  body.fill = 'green';
  const head = two.makeCircle(17, 0, 7);
  head.fill = 'black';
  const newBeetle = two.makeGroup(body, head);

  //const vectorLine = two.makeLine(0, 0, 0, 0);
  //vectorLines.push(vectorLine);

  return {
    beetle: newBeetle,
    selectedIndicator: selectedIndicator,
  };
}

function createFood() {
  const newFood = two.makeRectangle(0, 0, 10, 10);
  newFood.fill = 'Tomato';
  return newFood;
}

function drawBeetle(beetle, index) {
  const visualBeetleData = visualBeetles[index];
  const visualBeetle = visualBeetleData.beetle;

  const selectedIndicator = visualBeetleData.selectedIndicator;
  selectedIndicator.translation.set(beetle.position.x, beetle.position.y);

  if (beetle.selected) {
    selectedIndicator.visible = true;
  }
  else {
    selectedIndicator.visible = false;
  }

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

function getViewportDimensions() {
  return {
    width: document.documentElement.clientWidth,
    height: document.documentElement.clientHeight,
  };
}
