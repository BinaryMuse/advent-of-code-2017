const readWholeStream = require('./read-whole-stream.js');

function getRowLimits(row) {
  let smallest = row[0];
  let largest = row[0];

  row.forEach(num => {
    smallest = Math.min(smallest, num);
    largest = Math.max(largest, num);
  });

  return {smallest, largest};
}

function getNumDenom(row) {
  let pairs = [];
  row.forEach((numerator, i) => {
    row.forEach((denominator, j) => {
      if (i === j) return;
      pairs.push([numerator, denominator]);
    });
  });

  const [numerator, denominator] = pairs.find(pair => pair[0] % pair[1] === 0);
  return {numerator, denominator};
}

function checksumPart1(spreadsheet) {
  return spreadsheet.reduce((sum, row) => {
    const {smallest, largest} = getRowLimits(row);
    const diff = Math.abs(largest - smallest);
    return sum + diff;
  }, 0);
}

function checksumPart2(spreadsheet) {
  return spreadsheet.reduce((sum, row) => {
    const {numerator, denominator} = getNumDenom(row);
    const div = numerator / denominator;
    return sum + div;
  }, 0);
}

async function run() {
  const input = await readWholeStream(process.stdin);

  const spreadsheet = input.split('\n')
    .filter(line => line.trim())
    .map(line => line.split(/\W+/).map(s => parseInt(s, 10)));

  // console.log(checksumPart1(spreadsheet));
  console.log(checksumPart2(spreadsheet));
}

run();
