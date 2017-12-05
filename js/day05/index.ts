import * as fs from 'fs'

type JumpModifier = (n: number) => number

class JumpMaze {
  private offsets: number[]
  private current: number
  private steps: number
  private jumpModifier: JumpModifier

  constructor(offsets: number[], jumpModifier: JumpModifier = n => n + 1) {
    this.offsets = offsets.slice()
    this.jumpModifier = jumpModifier
    this.current = 0
    this.steps = 0
  }

  public isExited(): boolean {
    return this.current >= this.offsets.length
  }

  public jump(): void {
    const currentOffset = this.offsets[this.current]
    this.offsets[this.current] = this.jumpModifier(currentOffset)
    this.current += currentOffset
    this.steps++
  }

  public traverse(callback: (steps: number) => void): number {
    while(!this.isExited()) {
      this.jump()
      callback(this.getSteps())
    }

    return this.getSteps()
  }

  public getSteps(): number {
    return this.steps
  }
}

const inputFile: string = process.argv[2]
const input: string = fs.readFileSync(inputFile, {encoding: 'utf8'}).trim()
const offsets = input.split(/\r?\n/).map(s => parseInt(s, 10))

function part1() {
  console.log('Part 1')
  const maze = new JumpMaze(offsets)
  const steps = maze.traverse(step => step % 10000 === 0 && process.stdout.write('.'))
  console.log(`\nReached the end in ${steps} steps`)
}

function part2() {
  console.log('Part 2')
  const maze = new JumpMaze(offsets, n => n >= 3 ? n - 1 : n + 1)
  const steps = maze.traverse(step => step % 10000 === 0 && process.stdout.write('.'))
  console.log(`\nReached the end in ${steps} steps`)
}

part1()
part2()
