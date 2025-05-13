import { basename } from 'path';
import { sortFunctions, execute } from './sortExecute.ts';


const dataTypes: Record<string, (length: number) => number[]> = {
  'uniform': (length) => Array(length).fill(0),
  'repeating': (length) => {
    let max = Math.floor(Math.sqrt(length));
    return Array.from({ length }, () => Math.floor(Math.random() * max));
  },
  'random': (length) => Array.from({ length }, () => Math.floor(Math.random() * length)),
  'unique': (length) => Array.from({ length }, (_, i) => i).sort(() => Math.random() - 0.5),
  'ascending': (length) => Array.from({ length }, (_, i) => i),
  'descending': (length) => Array.from({ length }, (_, i) => length - 1 - i),
};

const testSizes = {
  'tiny': 4,
  'small': 9,
  'medium': 400,
  'large': 10000,
};

/**
 * Compares the different sorting algorithm implementations and the Javascript array.sort function.
 * The algorithms are executed with different data types and sizes and the results are printed.
 */
function compare(): void {
  // identify the algorithms ready for testing by executing a known test case
  let sortNames: Array<keyof typeof sortFunctions> = [];
  for (const [name, sort] of Object.entries(sortFunctions)) {
    try {
      const output = execute(sort, [3, 1, 4, 1, 5, 9, 2, 6, 5])[0];
      if (JSON.stringify(output) !== JSON.stringify([1, 1, 2, 3, 4, 5, 5, 6, 9])) {
        throw new Error();
      }
      sortNames.push(name as keyof typeof sortFunctions);
    } catch (error) {
      console.log(`${name} is not ready for comparison.`);
    }
  }

  // make a paragraph for each data type, a line for each algorithm and a column for each size
  // - for getting placement information, we execute all algortithms before printing
  for (const [dataType, dataGenerator] of Object.entries(dataTypes)) {
    let dataTypeStr = `${dataType} data (e.g. ${dataGenerator(4).join(', ')})`;
    console.log(`\nTesting ${dataTypeStr} of sizes`, Object.values(testSizes).join(', '));
    let sortResults: string[][] = Array.from({ length: sortNames.length }, () => []);
    for (const [sizeName, size] of Object.entries(testSizes)) {
      let inputData = dataGenerator(size);
      let expected = [...inputData].sort((a, b) => a - b);
      let testResults = [];
      for (let [stageId, sortName] of sortNames.entries()) {
        let [output, count, countStr]: [number[], number | null, string] = [[], null, ''];
        try {
          let sort = sortFunctions[sortName];
          [output, count, countStr] = execute(sort, inputData, 3600 * 24 * 10);
          if (JSON.stringify(output) !== JSON.stringify(expected)) {
            [count, countStr] = [null, 'wrong result'];
          }
        } catch (error) {
          [count, countStr] = [null, 'time/error'];
        }
        sortResults[stageId].push(countStr.padStart(10));
        testResults.push(count);
      }
      let positions = getPositions(testResults);
      for (let stageId = 0; stageId < sortNames.length; stageId++) {
        sortResults[stageId][sortResults[stageId].length - 1] += ` (${positions[stageId]})`;
      }
    }
    for (let stageId = 0; stageId < sortNames.length; stageId++) {
      console.log(`${sortNames[stageId]}: ${sortResults[stageId].join(', ')}`);
    }
  }
}

function getPositions(scores: Array<number | null>): number[] {
  let [pos, count, last]: [number, number, number | null] = [1, 0, 0];
  let positions: number[] = Array(scores.length).fill(0);
  for (let [i, score] of Array.from(scores.entries()).sort(
    // this comparator sorts accending and puts null values at the end
    (a, b) => a[1] === null ? (b[1] === null ? 0 : 1) : (b[1] === null ? -1 : a[1] - b[1])
  )) {
    if (count === 0 || last !== score) {
      [pos, count, last] = [pos + count, 0, score];
    }
    count++;
    positions[i] = pos;
  }
  return positions;
}

function printUsage() {
  console.log(
    '\nCompare the different sorting algorithm implementations and the Javascript Array.sort function.'
  );
  let scriptName = basename(process.argv[1]);
  console.log(`Usage: ${scriptName}`);
}


if (process.argv[1] === import.meta.filename) {
  if (process.argv.length != 2) {
    printUsage();
    process.exit(1);
  }
  compare();
}
