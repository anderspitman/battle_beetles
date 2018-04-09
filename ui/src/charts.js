const Two = require('two.js');
const d3 = require('d3');

const utils = require('./utils.js');

// colors taken from the fantastic Color Brewer: http://colorbrewer2.org
const COLORS = [
  '#e41a1c', // red
  '#377eb8', // blue
  '#4daf4a', // green
  '#984ea3', // purple
  '#ff7f00', // orange
  '#a65628',
  '#ffff33', // yellow
  '#f781bf'
];

const GRAPH_COLORS = [
  '#1f78b4', // dark blue
  '#33a02c', // dark green
  '#e31a1c', // dark red
  '#ff7f00', // dark orange
  '#6a3d9a', // dark purple
  '#ffff99', // yellow
  '#b15928', // brown
  '#a6cee3', // light blue
  '#b2df8a', // light green
  '#fb9a99', // light red
  '#fdbf6f', // light orange
  '#cab2d6', // light purple
]

function translate(x, y) {
  return "translate(" + x + "," + y + ")";
}

class Chart {

  constructor({
    title,
    domElementId,
  }) {

    this.elem = document.getElementById(domElementId);

    while(this.elem.firstChild) {
      this.elem.removeChild(this.elem.firstChild);
    }

    const text = d3.select(this.elem)
      .append('div')
        .attr('class', 'chart__title')
        .text(title)

    this.container = d3.select(this.elem)
      .append('div')
      .attr('class', 'chart__container')

    this.titleDim = text.node().getBoundingClientRect();

    const dim = this.elem.getBoundingClientRect();

    this.width = dim.width;
    this.height = dim.height - this.titleDim.height;

    this.centerX = this.width / 2;
    this.centerY = this.height / 2;
  }
}


class TwoJsChart extends Chart {
  constructor({
    title,
    domElementId,
  }) {

    super({ title, domElementId });

    const params = {
      width: this.width,
      height: this.height,
      //type: Two.Types.webgl,
    };

    this.two = new Two(params).appendTo(this.container.node());
  }
}

class ScatterPlot extends TwoJsChart {

  constructor({
    title,
    domElementId,
    yMin,
    yMax,
    xMin,
    xMax,
    maxPoints,
    variableNames,
    xLabel,
    yLabel,
    symbolSize,
    threshold,
    legend
  }) {

    super({ title, domElementId });

    this.yMax = yMax;
    this.numVariables = variableNames.length;
    this.variableNames = variableNames;

    this.margins = {
      left: 50,
      right: 30,
      top: 10,
      bottom: 45,
    };


    xMin = xMin === undefined ? 0 : xMin;
    xMax = xMax === undefined ? maxPoints : xMax;

    this.symbolSize = symbolSize === undefined ? 2 : symbolSize;
    this.threshold = threshold;
    this.legend = legend === undefined ? false : legend;

    this.xScale = d3.scaleLinear()
      .domain([xMin, xMax])
      .range([this.margins.left, this.width - this.margins.right]);

    this.yScale = d3.scaleLinear()
      .domain([yMin, yMax])
      .range([this.height - this.margins.bottom, this.margins.top])

    this.data = [];
    //this.points = [];

    const background =
      this.two.makeRectangle(
        this.margins.left + (this.adjustedWidth()/ 2),
        this.margins.top + (this.adjustedHeight() / 2),
        this.adjustedWidth(),
        this.adjustedHeight());

    background.fill = '#ededed';
    background.noStroke();


    this.xValues = [];
    this.yValues = [];

    this.valuesIndex = 0;

    this.symbols = [];

    for (let i = 0; i < this.numVariables; i++) {
      
      this.xValues.push(new Float32Array(maxPoints));
      this.yValues.push(new Float32Array(maxPoints));
      this.symbols.push([]);

      for (let j = 0; j < maxPoints; j++) {
        // initially render off-screen
        const point = this.two.makeCircle(-100, -100, this.symbolSize);
        point.fill = COLORS[i];
        //point.stroke = point.fill;
        point.noStroke();
        this.symbols[i].push(point);
      }

    }

    this.overlayContainer = this.container
      .append('svg')
        .attr('class', 'chart__axes-container')
        .attr('width', this.width)
        .attr('height', this.height)

    const xAxis = d3.axisBottom(this.xScale);
    this.overlayContainer
      .append('g')
        .attr("transform", "translate(0,"+(this.height-this.margins.bottom)+")")
        .call(xAxis)

    const yAxisLeft = d3.axisLeft(this.yScale);
    this.overlayContainer
      .append('g')
        .attr("transform", "translate("+(this.margins.left)+")")
        .call(yAxisLeft)

    const yAxisRight = d3.axisRight(this.yScale);
    this.overlayContainer
      .append('g')
        .attr("transform", "translate("+(this.width-this.margins.right)+")")
        .call(yAxisRight)

    // yLabel
    this.overlayContainer 
      .append("text")
        .attr("class", "chart__axis-label")
        .attr("transform", "rotate(-90)")
        .attr("x", -(this.margins.top + (this.adjustedHeight() / 2)))
        .attr("y", 15)
        .text(yLabel)
        .style("text-anchor", "middle")

    // xLabel
    this.overlayContainer 
      .append("text")
        .attr("class", "chart__axis-label")
        .attr("x", this.margins.left + (this.adjustedWidth() / 2))
        .attr("y", this.margins.top + this.adjustedHeight() + 35)
        .text(xLabel)
        .style("text-anchor", "middle")

    if (this.legend) {
      this.makeLegend();
    }

    // TODO: only used with the old render function
    // pre-allocate points offscreen
    //for (let i = 0; i < maxPoints; i++) {
    //  const point =
    //    this.two.makeCircle(this.width + 100, this.height + 100, 2);
    //  point.fill = this.color ? this.color : COLORS[1];
    //  point.stroke = point.fill;

    //  this.points.push(point);
    //}

    //this.two.bind('update', () => {
    //}).play();
    this.two.play();

  }

  makeLegend() {

    const g = this.overlayContainer
      .append("g")
        .attr("class", "chart__legend")
        .attr("transform", translate(this.width - 175, this.height - 300)) 

    const variable = g
      .selectAll(".chart__legend__variable")
        .data(COLORS.slice(0, this.numVariables))
      .enter()
      .append("g")
        .attr("class", "chart__legend__variable")
    
    variable.append("rect")
        .attr("width", 15)
        .attr("height", 15)
        .attr("y", (d, i) => i*20)
        .attr("fill", function(d) { return d; })

    variable.append("text")
        .attr("x", (d, i) => 20)
        .attr("y", (d, i) => i*20 + 12)
        .text((d, i) => this.variableNames[i])

  }

  adjustedWidth() {
    return this.width - this.margins.left - this.margins.right;
  }

  adjustedHeight() {
    return this.height - this.margins.top - this.margins.bottom;
  }

  update(data) {

    for (let i = this.data.length; i < data.length; i++ ) {
      this.data.push(data[i]);
    }

    this.render();
  }

  addPoints({ xVals, yVals }) {

    if (yVals.length < this.numVariables) {
      throw "not enough yVals for " + this.numVariables + " variables";
    }

    for (let i = 0; i < this.yValues.length; i++) {
      this.yValues[i][this.valuesIndex] = yVals[i];

      if (xVals === undefined) {
        this.xValues[i][this.valuesIndex] = this.valuesIndex;
      }
      else {
        this.xValues[i][this.valuesIndex] = xVals[i];
      }
    }
    ++this.valuesIndex;

    this.addPointsRender();
  }

  render() {

    for (let i = 0; i < this.data.length; i++) {

      const point = this.points[i];

      //const xRatio = i / this.data.length;
      //const xPos = this.margins.left +
      //  (xRatio * (this.width - this.margins.left - this.margins.right));
      const xPos = this.xScale(i);
      //const yRatio = this.data[i] / this.yMax;
      //// y is inverted
      //const yPos = this.height -
      //  (this.margins.top +
      //  (yRatio * (this.height - this.margins.top - this.margins.bottom)));
      const yPos = this.yScale(this.data[i]);

      point.translation.set(xPos, yPos);
    }
  }

  addPointsRender() {

    const lastAddedIndex = this.valuesIndex - 1;

    for (let i = 0; i < this.yValues.length; i++) {
      //const xPos = this.xScale(lastAddedIndex);
      const xPos = this.xScale(this.xValues[i][lastAddedIndex]);
      const yPos = this.yScale(this.yValues[i][lastAddedIndex]);
      this.symbols[i][lastAddedIndex].translation.set(xPos, yPos);

      if (this.threshold !== undefined) {

        this.symbols[i][lastAddedIndex].opacity = .6;

        if (this.yValues[i][lastAddedIndex] >= this.threshold) {
          this.symbols[i][lastAddedIndex].fill = COLORS[2];
        }
        else {
          this.symbols[i][lastAddedIndex].fill = COLORS[0];
        }
      }
    }
  }

  reset() {
    this.valuesIndex = 0;
  }
}


class Graph extends TwoJsChart {
  constructor({
    title,
    domElementId,
    vertices,
    edges,
  }) {
    super({ title, domElementId });

    this.edges = edges;

    const background =
      this.two.makeRectangle(
        this.centerX, this.centerY, this.width, this.height);
    background.fill = '#ededed';

    const group = this.two.makeGroup();
    group.translation.set(this.centerX, this.centerY);

    this.elem.addEventListener('wheel', (e) => {

      const zoomFactor = 0.2;

      if (e.deltaY > 0) {
        group.scale -= zoomFactor * group.scale;
      }
      else {
        group.scale += zoomFactor * group.scale;
      }

      this.two.update();
    });

    const sim = d3.forceSimulation(vertices)
     .force("charge", d3.forceManyBody().strength(-100))
     .force("link", d3.forceLink(edges).distance(100))
     .force("center", d3.forceCenter());

    this.visEdges = [];
    for (let edge of edges) {
      const newEdge = this.two.makeLine(0, 0, 10, 10);
      newEdge.stroke = '#bbbbbb';
      this.visEdges.push(newEdge);
      group.add(newEdge);
    }

    this.visVertices = [];
    for (let vertex of vertices) {
      const newVertex = this.two.makeCircle(0, 0, 10);
      newVertex.fill = GRAPH_COLORS[0];
      this.visVertices.push(newVertex);
      group.add(newVertex);
    }
    
    sim.on('tick', () => {

      for (let i = 0; i < this.visEdges.length; i++) {
        const edge = this.visEdges[i];
        const [anchor1, anchor2] = edge.vertices;
        // TODO: use this method for updating:
        // https://github.com/jonobr1/two.js/issues/271
        anchor1.set(edges[i].source.x, edges[i].source.y);
        anchor2.set(edges[i].target.x, edges[i].target.y);
      }

      for (let i = 0; i < this.visVertices.length; i++) {
        const vertex = this.visVertices[i];
        vertex.translation.set(vertices[i].x, vertices[i].y);
      }

      this.two.update();
    });

    this.two.update();
  }

  update(colorIndices, maxIndividual) {

    for (let i = 0; i < this.visVertices.length; i++) {
      this.visVertices[i].fill = GRAPH_COLORS[colorIndices[i]];
    }

    for (let i = 0; i < this.visEdges.length; i++) {

      const sourceIndex = this.edges[i].source.index;
      const targetIndex = this.edges[i].target.index;

      if (this.visVertices[sourceIndex].fill ===
          this.visVertices[targetIndex].fill) {
        this.visEdges[i].stroke = 'red';
        this.visEdges[i].linewidth = 5;
      }
      else {
        this.visEdges[i].stroke = '#bbbbbb';
        this.visEdges[i].linewidth = null;
      }
    }

    this.two.update();
  }
}

class DiversityPlot extends Chart {
  constructor({
    title,
    domElementId,
    numGenerations,
    maxValue,
  }) {

    super({ title, domElementId });

    this.elem = document.getElementById(domElementId);
    this.canvas = document.createElement('canvas');
    this.canvas.width = this.width;
    this.canvas.height = this.height;
    this.elem.appendChild(this.canvas);

    //this.scale = d3.scaleOrdinal(COLORS.slice(0, 4))
    //  .domain([0, 1]);

    const ctx = this.canvas.getContext('2d');
    this.ctx = ctx;

    this.reset();

    this.numGenerations = numGenerations;
  }

  appendGeneration(diversityData) {

    // TODO: figure out if there's any way sorting could make canvas run
    // faster (or slower), ie from not having to move the ctx as far between
    // draws? idk probably not an issue
    //diversityData.sort();

    const ctx = this.ctx;

    //ctx.clearRect(0, 0, this.width, this.height);

    ctx.globalAlpha = 0.1;

    ctx.fillStyle = COLORS[3];

    const ySize = this.height / this.numGenerations;

    const yPos = this.generationIndex * ySize;

    for (let i = 0; i < diversityData.length; i++) {
      const xPos = diversityData[i] * this.width;
      ctx.fillRect(xPos, yPos, 2, ySize);
    }

    ctx.globalAlpha = 1.0;

    ++this.generationIndex;
  }

  reset() {
    this.ctx.clearRect(0, 0, this.width, this.height);
    this.ctx.strokeRect(0, 0, this.width - 1, this.height - 1);
    this.generationIndex = 0;
  }
}

module.exports = {
  ScatterPlot,
  Graph,
  DiversityPlot,
  COLORS,
  GRAPH_COLORS,
};
