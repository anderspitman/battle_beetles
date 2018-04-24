import { MessageService } from './message_service';
import * as messages from './gen/messages_pb';
//import * as Two from 'two.js';
import * as Charts from './charts';
import * as d3 from 'd3';

const canvas = document.getElementById('canvas');
const rightPanel = document.getElementById('right-panel');
const stopButton = document.getElementById('stop-button');
const addBeetleButton = document.getElementById('add-beetle-button');
const speedSimButton = document.getElementById('speed-sim-button');
const battleSimButton = document.getElementById('battle-sim-button');
const createFormationButton = document.getElementById('create-formation-button');
const DEGREES_PER_RADIAN = 57.2958;

const viewportDimensions = getViewportDimensions();
const buttonRowHeight = 50;

const beetleDim = {
  width: 20,
  length: 20,
  headRadius: 7,
};

const params = {
  //width: viewportDimensions.width,
  width: rightPanel.clientWidth,
  height: viewportDimensions.height - buttonRowHeight,
  //height: rightPanel.clientHeight,
  //type: Two.Types.webgl,
};
//const two = new Two(params).appendTo(canvas);

const svg = d3.select(canvas)
  .append('svg')
    .attr('class', 'svg-canvas')
    .attr('width', rightPanel.clientWidth)
    .attr('height', rightPanel.clientHeight)
const canvasRect = canvas.getBoundingClientRect();

renderBackground();

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

const phenotypeChart = new Charts.ScatterPlot({
  title: "Phenotypes",
  xLabel: "Generation",
  yLabel: "Average Phenotype Values",
  domElementId: 'chart-pheno',
  yMin: 0,
  yMax: 100,
  maxPoints: numGenerations,
  variableNames: [
    "Avg Speed",
    "Avg Max Health",
    "Avg Attack Power",
    "Avg Food Collected",
    "Avg Size",
  ],
  legend: true,
});

const genotypeChart = new Charts.ScatterPlot({
  title: "Genotypes",
  xLabel: "Generation",
  yLabel: "Average Genotype Values",
  domElementId: 'chart-genes',
  yMin: 0,
  yMax: 1,
  maxPoints: numGenerations,
  variableNames: [
    "Avg Density",
    "Avg Strength",
    "Avg Quickness",
    "Avg Venomosity",
    "Avg Mandible Sharpness",
    "Avg Body Width",
    "Avg Body Length",
  ],
  legend: true,
});

phenotypeChart.reset();
genotypeChart.reset();

const messageService = new MessageService();
const socket = messageService.getSocket();
socket.binaryType = 'arraybuffer';

let dragging = false;
let dragStart = { x: 0.0, y: 0.0 };
let dragEnd = { x: 0.0, y: 0.0 };
//drawBackground();
//const selecticle = createSelecticle();

socket.onmessage = (event) => {

  const uiUpdate = messages.UiUpdate.deserializeBinary(event.data);
  //const messageBuffer = new Uint8Array(event.data);
  //const messageType = messageBuffer[0];
  //const message = messageBuffer.slice(1);

  if (uiUpdate.hasGameState()) {
    handleStateUpdate(uiUpdate.getGameState());
  }
  //else if (uiUpdate.hasCharts()) {
  //  handleChartsUpdate(uiUpdate.getCharts());
  //}
  else if (uiUpdate.hasChartsIncremental()) {
    handleChartsIncremental(uiUpdate.getChartsIncremental());
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
  messageService.createBeetle({ x: 0.0, y: 0.0 });
}

speedSimButton.onclick = (e) => {
  phenotypeChart.reset();
  genotypeChart.reset();
  messageService.runSpeedSimulation();
}

battleSimButton.onclick = (e) => {
  phenotypeChart.reset();
  genotypeChart.reset();
  messageService.runBattleSimulation();
}

createFormationButton.onclick = (e) => {
  messageService.createFormation();
}

function renderBackground() {

  // draw background
  const background = svg.append('g')
    .attr('class', 'background')
    .on('contextmenu', (d) => {
      d3.event.preventDefault();
      messageService.selectedMoveCommand({
        // accounts for where the canvas is on the page
        x: d3.event.clientX - canvasRect.left,
        y: d3.event.clientY - canvasRect.top,
      });
    })
    .on('mousedown', (d) => {
      dragging = true;
      dragStart = getWorldPosition(d3.event)
      d3.event.preventDefault();
    })
    .on('mouseup', (d) => {
      const LEFT_MOUSE_BUTTON_ID = 0;
      if (d3.event.button === LEFT_MOUSE_BUTTON_ID) {
        dragEnd = getWorldPosition(d3.event)
        dragging = false;
        messageService.selectAllInArea({
          x1: dragStart.x,
          y1: dragStart.y,
          x2: dragEnd.x,
          y2: dragEnd.y, 
        })
      }
    })

  background.append('rect')
    .attr('width', rightPanel.clientWidth)
    .attr('height', rightPanel.clientHeight)
    .attr('fill',   '#c98c5a')
}

function renderHomeBases(bases) {
  const baseWidth = 128;
  const baseHeight = baseWidth;

  const baseUpdate = svg.selectAll('.base')
    .data(bases)

  const baseEnter = baseUpdate.enter()
    .append('g')
      .attr('class', 'base')
      .on('contextmenu', (d) => {
        d3.event.preventDefault();
        messageService.selectedInteractCommand({ targetId: d.getId() })
      })

  const mainArea = baseEnter
    .append('rect')
      .attr('class', 'base__main-area')

  baseUpdate
      .attr('transform', (d) => {
        return 'translate('+d.getX()+', '+d.getY()+')';
      })

  mainArea 
      .attr('x', -(baseWidth / 2))
      .attr('y', -(baseHeight / 2))
      .attr('width', baseWidth)
      .attr('height', baseHeight)
      .attr('fill', '#724100')

  baseUpdate.exit().remove();
}

function renderFoodSources(foods) {
  const width = 64;
  const height = width;

  const update = svg.selectAll('.food')
    .data(foods)

  const enter = update.enter()
    .append('g')
      .attr('class', 'food')
      .on('contextmenu', (d) =>{
        d3.event.preventDefault();
        messageService.selectedInteractCommand({ targetId: d.getId() })
      })

  const mainArea = enter
    .append('rect')
      .attr('class', 'food__main-area')

  update
      .attr('transform', (d) => {
        return 'translate('+d.getX()+', '+d.getY()+')';
      })

  mainArea 
      .attr('x', -(width / 2))
      .attr('y', -(height / 2))
      .attr('width', width)
      .attr('height', height)
      .attr('fill', '#efc85d')

  update.exit().remove();
}

function renderBeetles(beetles) {
  const beetleUpdate = svg.selectAll('.beetle')
    .data(beetles)

  const beetleEnter = beetleUpdate.enter()
    .append('g')
      .attr('class', 'beetle')
      .on('click', (d) => {
        console.log(d.getBodyWidth(), d.getBodyLength());
        if (!shiftKeyDown) {
          messageService.deselectAllBeetles()
        }
        messageService.selectBeetle({ beetleId: d.getId() });
      })
      .on('contextmenu', (d) => {
        d3.event.preventDefault();
        messageService.selectedInteractCommand({ targetId: d.getId() })
      })

  const head = beetleEnter
    .append('circle')
      .attr('class', 'beetle__head')
  const body = beetleEnter
    .append('rect')
      .attr('class', 'beetle__body')
  beetleEnter
    .append('rect')
      .attr('class', 'beetle__selected-indicator')
      .attr('width', 50)
      .attr('height', 50)
      .attr('x', -25)
      .attr('y', -25)
      .attr('fill', 'none')
      .attr('stroke', 'lightgreen')
      .attr('visibility', 'hidden')

  head 
      .attr('r', (d) => calcHeadRadius(d.getBodyWidth()))
      .attr('cx', (d) => {
        return (d.getBodyLength() / 2) + calcHeadRadius(d.getBodyWidth());
      })
      .attr('fill', '#1c1c1c')

  beetleUpdate
      .attr('transform', (d) => {
        const deg =  d.getAngle() * DEGREES_PER_RADIAN;
        return 'translate('+d.getX()+', '+d.getY()+') ' + 'rotate('+deg+') '
      })

  const bodyUpdate = beetleUpdate
    .select('.beetle__body')

  bodyUpdate
      .attr('x', (d) => -d.getBodyLength() / 2)
      .attr('y', (d) => -d.getBodyWidth() / 2)
      .attr('width', (d) => {
        return d.getBodyLength()
      })
      .attr('height', (d) => d.getBodyWidth())
      .attr('fill', (d) => {
        const color = d.getColor();
        const r = color.getR();
        const g = color.getG();
        const b = color.getB();
        const a = color.getA();
        return 'rgba('+r+','+g+','+b+','+a+')';
      })

  const selectedIndicatorUpdate = beetleUpdate
    .select('.beetle__selected-indicator')

  selectedIndicatorUpdate
      .attr('visibility', (d) => d.getSelected() ? 'visible' : 'hidden')
      .attr('transform', (d) => 'rotate('+(-d.getAngle() * DEGREES_PER_RADIAN)+')')

  beetleUpdate.exit().remove();
}

function calcHeadRadius(bodyWidth) {
  return bodyWidth / 3;
}

function handleStateUpdate(gameState) {
  const beetles = gameState.getBeetlesList();
  const bases = gameState.getHomeBasesList();
  const foods = gameState.getFoodSourcesList();

  renderHomeBases(bases);
  renderFoodSources(foods);
  renderBeetles(beetles);
}

function handleChartsIncremental(msg) {
  const avgSpeed = msg.getAvgSpeed();

  phenotypeChart.addPoints({
      yVals: [
        msg.getAvgSpeed(),
        msg.getAvgMaxHealth(),
        msg.getAvgAttackPower(),
        msg.getAvgFoodCollected(),
        msg.getAvgSize(),
      ],
  });

  genotypeChart.addPoints({
      yVals: [
        msg.getAvgSize(),
        msg.getAvgCarapaceDensity(),
        msg.getAvgStrength(),
        msg.getAvgQuickness(),
        msg.getAvgVenomosity(),
        msg.getAvgMandibleSharpness(),
        msg.getAvgBodyWidth(),
        msg.getAvgBodyLength(),
      ],
  });
}

function handleChartsUpdate(chartsMessage) {

  const avgSpeeds = chartsMessage.getAvgSpeedsList();
  const maxFitnessList = chartsMessage.getMaxFitnessesList();
  const avgSizeList = chartsMessage.getAverageSizesList();
  const avgDensityList = chartsMessage.getAverageDensitiesList();
  const avgStrengthList = chartsMessage.getAverageStrengthsList();
  const avgQuicknessList = chartsMessage.getAverageQuicknessesList();
  
  for (let i = 0; i < avgSpeeds.length; i++) {

    phenotypeChart.addPoints({
      yVals: [
        avgSpeeds[i].getValue(),
        maxFitnessList[i].getValue()
      ],
    });

    genotypeChart.addPoints({
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
        vis[i].obj.visible = true;
      }
      else {
        vis.push(createNew());
      }
    }
  }
  else if (vis.length >= model.length) {

    for (let i = 0; i < model.length; i++) {
        vis[i].obj.visible = true;
    }

    for (let i = model.length; i < vis.length; i++) {
      vis[i].obj.visible = false;

      if (vis[i].selectedIndicator) {
        vis[i].selectedIndicator.visible = false;
      }
    }
  }
}

function createSelecticle() {
  const selecticle = two.makeRectangle(0, 0, 50, 50);
  selecticle.stroke = 'black';
  selecticle.fill = 'none';
  return selecticle;
}

function getWorldPosition(e) {
    return {
      x: e.clientX - canvasRect.left,
      y: e.clientY - canvasRect.top
    }
}

function createBeetle() {

  const selectedIndicator = two.makeRectangle(0, 0, 50, 50);
  selectedIndicator.stroke = 'lightgreen';
  selectedIndicator.fill = 'none';

  const body = two.makeRectangle(0, 0, beetleDim.width, beetleDim.length);
  body.fill = '#679b50';
  const head = two.makeCircle(17, 0, beetleDim.headRadius);
  head.fill = '#1c1c1c';
  const newBeetle = two.makeGroup(body, head);

  //const vectorLine = two.makeLine(0, 0, 0, 0);
  //vectorLines.push(vectorLine);

  return {
    obj: newBeetle,
    selectedIndicator: selectedIndicator,
    body: body,
  };
}

function drawBeetle(beetle, index) {
  const visualBeetleData = visualBeetles[index];
  const visualBeetle = visualBeetleData.obj;
  const body = visualBeetleData.body;

  // use width for scale heuristic
  const scale = beetle.getBodyLength() / beetleDim.length;
  //if (index === 0) {
  //  console.log(beetle.getBodyLength(), scale);
  //}

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

  const color = beetle.getColor();
  const r = color.getR();
  const g = color.getG();
  const b = color.getB();
  const a = color.getA();
  body.fill = 'rgba('+r+','+g+','+b+','+a+')';

  //const line = vectorLines[index];
  //const [anchor1, anchor2] = line.vertices;
  //anchor1.set(beetle.position.x, beetle.position.y);
  //anchor2.set(beetle.position.x + (beetle.direction.x * 50),
  //  beetle.position.y + (beetle.direction.y * 50));
}

function getViewportDimensions() {
  return {
    width: document.documentElement.clientWidth,
    height: document.documentElement.clientHeight,
  };
}
