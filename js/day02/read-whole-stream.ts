import {Stream} from 'stream'

export default function readWholeStream(stream: Stream) {
  return new Promise(resolve => {
    let buffer = ''

    stream.on('data', (chunk) => {
      buffer += chunk.toString()
    })

    stream.on('end', () => resolve(buffer))
  })
}
