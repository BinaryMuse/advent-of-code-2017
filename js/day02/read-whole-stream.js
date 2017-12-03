module.exports = function readWholeStream(stream) {
  return new Promise(resolve => {
    let buffer = '';

    stream.on('data', (chunk) => {
      buffer += chunk.toString();
    });

    stream.on('end', () => resolve(buffer));
  });
}
