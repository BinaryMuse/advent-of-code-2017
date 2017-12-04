const sequence = process.argv[2]

const numbers: Array<number> = sequence.split('').map(str => parseInt(str, 10))

type GetNextIdx = (ary: Array<number>, idx: number) => number

function sumMatching(numbers: Array<number>, getNextIdx: GetNextIdx = (ary, idx) => idx + 1) {
  return numbers.reduce((sum, num, i) => {
    let nextIdx = getNextIdx(numbers, i)
    if (nextIdx >= numbers.length) {
      nextIdx -= numbers.length
    }
    const next = numbers[nextIdx]
    return num === next ? sum + num : sum
  }, 0)
}

// part 1
// const result = sumMatching(numbers)
// part 2
const result = sumMatching(numbers, (ary, idx) => idx + ary.length / 2)

console.log(result)
