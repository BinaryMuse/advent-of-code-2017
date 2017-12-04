import * as fs from 'fs'

type WordComparePredicate = (word1: Word, word2: Word) => boolean

class Word {
  private text: string

  constructor(text: string) {
    this.text = text
  }

  getText(): string {
    return this.text
  }

  isEqualTo(otherWord: Word): boolean {
    return this.text === otherWord.getText()
  }

  isAnagramOf(otherWord: Word): boolean {
    const thisSortedText = this.text.split('').sort().join()
    const otherSortedText = otherWord.getText().split('').sort().join()

    return thisSortedText === otherSortedText
  }
}

class Line {
  private words: Array<Word>

  constructor(text: string) {
    this.words = text.split(/\W+/).map(t => new Word(t))
  }

  compareAllWords(predicate: WordComparePredicate): boolean {
    return this.words.some(word1 => {
      return this.words.some(word2 => {
        return word1 !== word2 && predicate(word1, word2)
      })
    })
  }

  containsDuplicateWords(): boolean {
    return this.compareAllWords((word1, word2) => word1.isEqualTo(word2))
  }

  containsAnagrams(): boolean {
    return this.compareAllWords((word1, word2) => word1.isAnagramOf(word2))
  }
}

const inputFile: string = process.argv[2]
const input: string = fs.readFileSync(inputFile, {encoding: 'utf8'})

const lines: Array<Line> = input.split(/\r?\n/)
  .filter(text => !!text.trim())
  .map(text => new Line(text))

console.log(`Part 1: ${lines.filter(line => !line.containsDuplicateWords()).length}`)
console.log(`Part 2: ${lines.filter(line => !line.containsAnagrams()).length}`)
