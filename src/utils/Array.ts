let splitArray = (array: number[], width: number, height: number): number[][] => {
  if(height * width !== array.length) throw new Error("Input array must match product of width and height");
  let output: number[][] = new Array(height);
  for (let i = 0; i < height; i++) {
    output[i] = new Array(width);
    for (let j = 0; j < width; j++) {
      output[i][j] = array[i * width + j];
    }
  }
  return output
}

export { splitArray }