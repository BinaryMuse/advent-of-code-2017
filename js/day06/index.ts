import * as fs from 'fs'

class MemoryAllocator {
  private banks: number[]
  private visited: Set<string> = new Set<string>()
  private history: string[] = []
  private steps: number = 0

  constructor(banks: number[]) {
    this.banks = banks
  }

  public balance(): void {
    this.addToHistory()
    const maxBankSize = Math.max(...this.banks)
    const bankIndex = this.banks.findIndex(val => val === maxBankSize)
    this.banks[bankIndex] = 0
    this.redistribute(maxBankSize, bankIndex + 1)
    this.steps++
  }

  private redistribute(amount: number, index: number) {
    if (index >= this.banks.length) index = 0
    if (amount > 0) {
      this.banks[index]++
      this.redistribute(amount - 1, index + 1)
    }
  }

  private addToHistory() {
    const hash = this.getHash()
    this.visited.add(hash)
    this.history.push(hash)
  }

  public getHash(): string {
    return this.banks.join(',')
  }

  public hasVisited(hash: string): boolean {
    return this.visited.has(hash)
  }

  public getSteps(): number {
    return this.steps
  }

  public getStepsAtFirst(hash: string): number {
    const firstSaw = this.history.indexOf(hash)
    const diff = this.steps - firstSaw
    return diff
  }
}

const inputFile = process.argv[2]
const input = fs.readFileSync(inputFile, {encoding: 'utf8'}).trim()
const banks = input.split("\t").map(str => parseInt(str, 10))
const allocator = new MemoryAllocator(banks)

while (!allocator.hasVisited(allocator.getHash())) {
  allocator.balance()
}
console.log(`Steps: ${allocator.getSteps()}`)
console.log(`Loop size: ${allocator.getStepsAtFirst(allocator.getHash())}`)
