// math utils taken from: https://gist.github.com/Daniel-Hug/7273430
function sum(array) {
  var num = 0;
  for (var i = 0, l = array.length; i < l; i++) {
    num += array[i];
  }
  return num;
}

function calculateMean(array) {
  return sum(array) / array.length;
}

function calculateVariance(array) {
  const mean = calculateMean(array);
  return calculateMean(array.map(function(num) {
    return Math.pow(num - mean, 2);
  }));
}

// from: https://stackoverflow.com/a/10784675/943814
function replaceAt(s, n, t) {
    return s.substring(0, n) + t + s.substring(n + 1);
}

function deepCopy(obj) {
  return JSON.parse(JSON.stringify(obj));
}

module.exports = {
  sum,
  mean: calculateMean,
  variance: calculateVariance,
  replaceAt,
  deepCopy,
};
