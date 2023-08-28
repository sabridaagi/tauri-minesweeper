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

let contains = (array: number[][], searchValue: number): boolean => {
  for(let i = 0; i < array.length; i++) {
    for(let j = 0; j < array[i].length; j++) {
      if(array[i][j] === searchValue) return true;
    }
  }

  return false;
}

export { splitArray, contains }