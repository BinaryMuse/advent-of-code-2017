const target = parseInt(process.argv[2])

function findClosestSquare (target: number, lastBase = 1): {base: number, square: number} {
  const square = lastBase * lastBase
  if (square >= target) {
    return {base: lastBase, square}
  } else {
    return findClosestSquare(target, lastBase + 2)
  }
}

function findCoordinate (target: number, startCoord: Array<number>, square: number, base: number) {
  const period = base - 1
  let value = square
  let coord = startCoord

  if (target === value) {
    return coord
  }

  // Going left
  if (target >= value - period) {
    const diff = value - target
    coord = [coord[0] - diff, coord[1]]
    return coord
  }

  // Going up
  value -= period
  coord = [coord[0] - period, coord[1]]
  if (target >= value - period) {
    const diff = value - target
    coord = [coord[0], coord[1] + diff]
    return coord
  }

  // Going right
  value -= period
  coord = [coord[0], coord[1] + period]
  if (target >= value - period) {
    const diff = value - target
    coord = [coord[0] + diff, coord[1]]
    return coord
  }

  // Going down
  value -= period
  coord = [coord[0] + period, coord[1]]
  if (target > value - period) {
    const diff = value - target
    coord = [coord[0], coord[1] - diff]
    return coord
  }

  throw new Error('Expected to find target')
}

const {base, square} = findClosestSquare(target)
const armDepth = (base - 1) / 2
const coordinate = [armDepth, -armDepth]
const targetCoord = findCoordinate(target, coordinate, square, base)
const distance = Math.abs(targetCoord[0]) + Math.abs(targetCoord[1])
console.log(targetCoord, `Distance: ${distance}`)
