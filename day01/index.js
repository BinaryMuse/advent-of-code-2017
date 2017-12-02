const sequence = process.argv[2];

const numbers = sequence.split('').map(str => parseInt(str, 10));

function sumMatching(numbers, getNextIdx = (ary, idx) => idx + 1) {
  return numbers.reduce((sum, num, i) => {
    let nextIdx = getNextIdx(numbers, i);
    if (nextIdx >= numbers.length) {
      nextIdx -= numbers.length;
    }
    const next = numbers[nextIdx];
    return num === next ? sum + num : sum;
  }, 0);
}

// part 1
// const result = sumMatching(numbers);
// part 2
const result = sumMatching(numbers, (ary, idx) => idx + ary.length / 2);

console.log(result);
