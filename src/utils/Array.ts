let splitArray = (array: number[], width: number, height: number): number[][] => {
  if(width * height !== array.length) throw new Error("Input array must match product of width and height");
  let output: number[][] = new Array(width);
  for (let i = 0; i < width; i++) {
    output[i] = new Array(height);
    for (let j = 0; j < height; j++) {
      output[i][j] = array[i * height + j];
    }
  }
  return output
}

export { splitArray }