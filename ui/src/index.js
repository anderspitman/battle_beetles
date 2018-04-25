import { MessageService } from './message_service';
import * as messages from './gen/messages_pb';
//import * as Two from 'two.js';
import * as Charts from './charts';
import * as d3 from 'd3';

const canvas = document.getElementById('canvas');
const rightPanel = document.getElementById('right-panel');
const stopButton = document.getElementById('stop-button');
const addBeetleButton = document.getElementById('add-beetle-button');
//const speedSimButton = document.getElementById('speed-sim-button');
const battleSimButton = document.getElementById('battle-sim-button');
const foodGAButton = document.getElementById('food-ga-button');
const fightSimButton = document.getElementById('fight-sim-button');
const createFormationButton = document.getElementById('create-formation-button');
const DEGREES_PER_RADIAN = 57.2958;

const beetleDim = {
  width: 20,
  length: 20,
  headRadius: 7,
};

const svg = d3.select(canvas)
  .append('svg')
    .attr('class', 'svg-canvas')
    .attr('width', rightPanel.clientWidth)
    .attr('height', rightPanel.clientHeight)

const gameContainer = svg.append('g')
    .attr('class', 'game-container')

//svg.call(d3.zoom()
//    .scaleExtent([1 / 4, 8])
//    .on("zoom", zoomed))
//    .on("mousedown.zoom", null)
//    .on("touchstart.zoom", null)
//    .on("touchmove.zoom", null)
//    .on("touchend.zoom", null)

function zoomed() {
  gameContainer.attr("transform", d3.event.transform);
}

const canvasRect = canvas.getBoundingClientRect();

renderBackground();

gameContainer.append('g')
    .attr('class', 'beetles')

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

//const phenotypeChart = new Charts.ScatterPlot({
//  title: "Phenotypes",
//  xLabel: "Generation",
//  yLabel: "Average Phenotype Values",
//  domElementId: 'chart-pheno',
//  yMin: 0,
//  yMax: 100,
//  maxPoints: numGenerations,
//  variableNames: [
//    "Avg Speed",
//    "Avg Max Health",
//    "Avg Attack Power",
//    "Avg Food Collected",
//    "Avg Size",
//  ],
//  legend: true,
//});

const varNames = [
  "Avg Density",
  "Avg Strength",
  "Avg Quickness",
  "Avg Venomosity",
  "Avg Mandible Sharpness",
  "Avg Body Width",
  "Avg Body Length",
]

const genotypeChart = new Charts.ScatterPlot({
  title: "Genotypes",
  xLabel: "Generation",
  yLabel: "Average Genotype Values",
  domElementId: 'chart-genes',
  yMin: 0,
  yMax: 1,
  maxPoints: numGenerations,
  variableNames: varNames,
  //legend: true,
});

const geneBarChart = new Charts.BarChart({
  title: "Genotypes",
  domElementId: 'gene-bar-chart',
  yMin: 0,
  yMax: 1,
  maxPoints: numGenerations,
  variableNames: varNames,
})

const legendChart = new Charts.LegendChart({
  title: "Legend",
  domElementId: 'gene-legend-chart',
  variableNames: varNames,
});

//phenotypeChart.reset();
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

//speedSimButton.onclick = (e) => {
//  phenotypeChart.reset();
//  genotypeChart.reset();
//  messageService.runSpeedSimulation();
//}

battleSimButton.onclick = (e) => {
  //phenotypeChart.reset();
  genotypeChart.reset();
  messageService.runBattleSimulation();
}

foodGAButton.onclick = (e) => {
  //phenotypeChart.reset();
  genotypeChart.reset();
  messageService.runFoodGA();
}

fightSimButton.onclick = (e) => {
  //phenotypeChart.reset();
  genotypeChart.reset();
  messageService.runFightSimulation();
}

createFormationButton.onclick = (e) => {
  messageService.createFormation();
}

function renderBackground() {

  // draw background
  const background = gameContainer.append('g')
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
      //console.log(d3.event);
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
    //.attr('width', rightPanel.clientWidth)
    //.attr('height', rightPanel.clientHeight)
    .attr('width', 5000)
    .attr('height', 5000)
    .attr('x', -2500)
    .attr('y', -2500)
    .attr('fill',   '#c98c5a')
}

function renderHomeBases(bases) {
  const baseWidth = 128;
  const baseHeight = baseWidth;

  const baseUpdate = gameContainer.selectAll('.base')
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

  baseEnter
    .append('text')
      .attr('class', 'base__text')
      .attr('text-anchor', 'middle')
      .attr('alignment-baseline', 'central')
      .attr('font-size', 18)
      .attr('font-weight', 'bold')
      .attr('fill', '#eeeeee')
      .attr('font-family', 'Helvetica')

  baseUpdate
      .attr('transform', (d) => {
        return 'translate('+d.getX()+', '+d.getY()+')';
      })
    .select('.base__text')
      .text((d) => d.getFoodStoredAmount())

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

  const update = gameContainer.selectAll('.food')
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

  enter
    .append('text')
      .attr('class', 'food__text')
      .attr('text-anchor', 'middle')
      .attr('alignment-baseline', 'central')
      .attr('font-size', 16)
      .attr('font-weight', 'bold')
      .attr('font-family', 'Helvetica')

  update
      .attr('transform', (d) => {
        return 'translate('+d.getX()+', '+d.getY()+')';
      })
    .select('.food__text')
      .text((d) => d.getAmount())

  mainArea 
      .attr('x', -(width / 2))
      .attr('y', -(height / 2))
      .attr('width', width)
      .attr('height', height)
      .attr('fill', '#efc85d')

  update.exit().remove();
}

function renderBeetles(data) {

  const beetles = gameContainer.select('.beetles')
  const beetleUpdate = beetles.selectAll('.beetle')
    .data(data)

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

  beetleEnter
    .append('text')
      .attr('class', 'beetle__text')
      .attr('text-anchor', 'middle')
      .attr('alignment-baseline', 'central')
      .attr('font-size', 16)
      .attr('font-weight', 'bold')
      .attr('font-family', 'Helvetica')
      .attr('fill', '#eeeeee')

  head 
      .attr('fill', '#1c1c1c')

  beetleUpdate
      .attr('transform', (d) => {
        const deg =  d.getAngle() * DEGREES_PER_RADIAN;
        return 'translate('+d.getX()+', '+d.getY()+') ' + 'rotate('+deg+') '
      })
  const bodyUpdate = beetleUpdate
    .select('.beetle__body')
  const textUpdate = beetleUpdate
    .select('.beetle__text')
  const headUpdate = beetleUpdate
    .select('.beetle__head')

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

  headUpdate
      .attr('r', (d) => calcHeadRadius(d.getBodyWidth()))
      .attr('cx', (d) => {
        return (d.getBodyLength() / 2) + calcHeadRadius(d.getBodyWidth());
      })

  const selectedIndicatorUpdate = beetleUpdate
    .select('.beetle__selected-indicator')

  selectedIndicatorUpdate
      .attr('visibility', (d) => d.getSelected() ? 'visible' : 'hidden')
      .attr('transform', (d) => 'rotate('+(-d.getAngle() * DEGREES_PER_RADIAN)+')')

  textUpdate
      .attr('transform', (d) => 'rotate('+(-d.getAngle() * DEGREES_PER_RADIAN)+')')
      .attr('x', (d) => d.getBodyLength() / 2)
      .attr('y', (d) => -d.getBodyLength() / 2)
      .text((d) => d.getFoodCarrying())

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

  //phenotypeChart.addPoints({
  //    yVals: [
  //      msg.getAvgSpeed(),
  //      msg.getAvgMaxHealth(),
  //      msg.getAvgAttackPower(),
  //      msg.getAvgFoodCollected(),
  //      msg.getAvgSize(),
  //    ],
  //});

  const geneVals = [
    msg.getAvgCarapaceDensity(),
    msg.getAvgStrength(),
    msg.getAvgQuickness(),
    msg.getAvgVenomosity(),
    msg.getAvgMandibleSharpness(),
    msg.getAvgBodyWidth(),
    msg.getAvgBodyLength(),
  ]

  genotypeChart.addPoints({
      yVals: geneVals,
  });

  geneBarChart.update({ data: geneVals })
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

function getViewportDimensions() {
  return {
    width: document.documentElement.clientWidth,
    height: document.documentElement.clientHeight,
  };
}
