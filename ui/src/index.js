import { MessageService } from './message_service';
import * as messages from './gen/messages_pb';
import * as Two from 'two.js';
import * as Charts from './charts';

const canvas = document.getElementById('canvas');
const rightPanel = document.getElementById('right-panel');
const stopButton = document.getElementById('stop-button');
const addBeetleButton = document.getElementById('add-beetle-button');

const viewportDimensions = getViewportDimensions();
const buttonRowHeight = 50;

const params = {
  //width: viewportDimensions.width,
  width: rightPanel.clientWidth,
  height: viewportDimensions.height - buttonRowHeight,
  //height: rightPanel.clientHeight,
  //type: Two.Types.webgl,
};
const two = new Two(params).appendTo(canvas);

const canvasRect = canvas.getBoundingClientRect();

let shiftKeyDown = false;
window.onkeyup = function(e) {
  shiftKeyDown = false;
};
window.onkeydown = function(e) {
  if (e.key == 'Shift') {
    shiftKeyDown = true;
  }
};

// TODO: get from server
const numGenerations = 128;

const fitnessChart = new Charts.ScatterPlot({
  title: "Fitness (Speed)",
  xLabel: "Generation",
  yLabel: "Fitness",
  domElementId: 'chart-stats',
  yMin: 0,
  yMax: 10,
  maxPoints: numGenerations,
  variableNames: [
    "Average Fitness",
    "Max Fitness",
  ],
  legend: true,
});

const geneChart = new Charts.ScatterPlot({
  title: "Gene Expression",
  xLabel: "Generation",
  yLabel: "Expression Ratio",
  domElementId: 'chart-genes',
  yMin: 0,
  yMax: 1,
  maxPoints: numGenerations,
  variableNames: [
    "Size",
    "Density",
    "Strength",
    "Quickness",
  ],
  legend: true,
});

fitnessChart.reset();
geneChart.reset();

const visualBeetles = [];
const visualFoods = [];
//const vectorLines = [];

const messageService = new MessageService();
const socket = messageService.getSocket();
socket.binaryType = 'arraybuffer';

drawBackground();

socket.onmessage = (event) => {

  const uiUpdate = messages.UiUpdate.deserializeBinary(event.data);
  //const messageBuffer = new Uint8Array(event.data);
  //const messageType = messageBuffer[0];
  //const message = messageBuffer.slice(1);

  if (uiUpdate.hasGameState()) {
    handleStateUpdate(uiUpdate.getGameState());
  }
  else if (uiUpdate.hasCharts()) {
    handleChartsUpdate(uiUpdate.getCharts());
  }
}

socket.onopen = (event) => {
  //console.log("sending yolo")
  //socket.send("Yolo")
}

socket.onclose = (event) => {
  console.log("Websocket connection closed");
}

stopButton.onclick = (event) => {
  messageService.terminate();
}

addBeetleButton.onclick = (e) => {
  console.log("create beetle");
  messageService.createBeetle({ x: 0.0, y: 0.0 });
}

function handleStateUpdate(gameState) {

  const beetles = gameState.getBeetlesList();
  //console.log(beetles);
  //const data = JSON.parse(event.data);

  // convert beetles object to array
  //const beetles = Object.entries(data.beetles).map((tuple) => tuple[1]);
  //console.log(beetles);
  //const foods = data.food;
  //console.log(foods);

  matchArrays(beetles, visualBeetles, createBeetle);
  //matchArrays(foods, visualFoods, createFood);

  for (let i = 0; i < beetles.length; i++) {
    const beetle = beetles[i];

    if (visualBeetles[i].beetle._renderer && visualBeetles[i].beetle._renderer.elem) {
      visualBeetles[i].beetle._renderer.elem.onclick = (e) => {
        if (!shiftKeyDown) {
          messageService.deselectAllBeetles()
        }

        messageService.selectBeetle({ beetleId: beetle.getId() })
      };

      visualBeetles[i].beetle._renderer.elem.oncontextmenu = (e) => {
        e.preventDefault();
        messageService.selectedInteractCommand({ beetleId: beetle.getId() })
      };
    }

    drawBeetle(beetle, i);
  }

  //for (let i = 0; i < foods.length; i++) {
  //  const food = foods[i];
  //  drawFood(food, i);
  //}

  requestAnimationFrame(() => {
    two.update();
  });

}

function handleChartsUpdate(chartsMessage) {

  const avgFitnessList = chartsMessage.getAverageFitnessesList();
  const maxFitnessList = chartsMessage.getMaxFitnessesList();
  const avgSizeList = chartsMessage.getAverageSizesList();
  const avgDensityList = chartsMessage.getAverageDensitiesList();
  const avgStrengthList = chartsMessage.getAverageStrengthsList();
  const avgQuicknessList = chartsMessage.getAverageQuicknessesList();
  
  for (let i = 0; i < avgFitnessList.length; i++) {

    fitnessChart.addPoints({
      yVals: [
        avgFitnessList[i].getValue(),
        maxFitnessList[i].getValue()
      ],
    });

    geneChart.addPoints({
      yVals: [
        avgSizeList[i].getValue(),
        avgDensityList[i].getValue(),
        avgStrengthList[i].getValue(),
        avgQuicknessList[i].getValue(),
      ],
    });
  }
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
  else if (vis.length >= model.length) {

    for (let i = 0; i < model.length; i++) {
        vis[i].beetle.visible = true;
    }

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
    messageService.selectedMoveCommand({
      // accounts for where the canvas is on the page
      x: e.clientX - canvasRect.left,
      y: e.clientY - canvasRect.top,
    });
  };
}

const beetleDim = {
  width: 20,
  height: 20,
  headRadius: 7,
};

function createBeetle() {

  const selectedIndicator = two.makeRectangle(0, 0, 50, 50);
  selectedIndicator.stroke = 'lightgreen';
  selectedIndicator.fill = 'none';

  const body = two.makeRectangle(0, 0, beetleDim.width, beetleDim.height);
  body.fill = '#679b50';
  const head = two.makeCircle(17, 0, beetleDim.headRadius);
  head.fill = '#1c1c1c';
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

  // use width for scale heuristic
  const scale = beetle.getSize() / beetleDim.width;

  const selectedIndicator = visualBeetleData.selectedIndicator;
  selectedIndicator.translation.set(beetle.getX(), beetle.getY());
  selectedIndicator.scale = scale;

  if (beetle.getSelected()) {
    selectedIndicator.visible = true;
  }
  else {
    selectedIndicator.visible = false;
  }

  visualBeetle.translation.set(beetle.getX(), beetle.getY());
  visualBeetle.rotation = beetle.getAngle();
  visualBeetle.scale = scale;

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
