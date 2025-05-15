import { basename } from 'path';
import { readFile } from 'fs/promises';

import { ListElement } from './Element.ts';
import { Counter } from './Counter.ts';
import { sort as sort1 } from './stage1.ts';
import { sort as sort2 } from './stage2.ts';
import { sort as sort3 } from './stage3.ts';
import { sort as sort4 } from './stage4.ts';


const sortFunctions = {
  'stage1': sort1,
  'stage2': sort2,
  'stage3': sort3,
  'stage4': sort4,
  'jsarray': (list: ListElement[]) => list.sort(ListElement.compare),
}

/**
 * execute a sorting function on a test data set.
 * @param sort the sorting function to test.
 * @param input_data the data to sort.
 * @param limit the maximum number of comparison operations.
 * @return array of the sorted data, comparison count and comparison count string.
 */
function execute(
  sort: (elements: ListElement[]) => void,
  inputData: number[], 
  limit: number = 3600
): [number[], number, string] {
  let counter = new Counter(limit);
  let inputElements = inputData.map(i => new ListElement(counter, i));
  sort(inputElements);
  let [ count, countString ] = [counter.count, counter.toString()];
  let outputData = inputElements.map(element => element.toInteger());
  return [outputData, count, countString];
}

/**
 * load the test data from a file.
 * @param path file path.
 * @returns a promise of a list of test sets.
 */
async function loadData(path: string): Promise<number[][]> {
  let content = await readFile(path, 'utf-8');
  let structure = JSON.parse(content);
  if (!Array.isArray(structure)) {
    throw new Error(`Cannot use ${path}. it is not an array.`);
  }
  for (let [i, item] of structure.entries()) {
    if (!Array.isArray(item) || !item.every(i => Number.isInteger(i))) {
      throw new Error(`Cannot use ${path}. item ${i} is not an array of integers.`);
    }
  }
  return structure;
}

function* serializeData(data: number[][]): Generator<string> {
  yield '[';
  for (let [i, item] of data.entries()) {
    let sep = i < data.length - 1 ? ',' : '';
    yield '  [' + item.join(', ') + ']' + sep;
  }
  yield ']';
}

function printUsage() {
  console.log('\nExecute a sort algorithm with some test data');
  let scriptName = basename(process.argv[1]);
  let algorithms = '{' + Object.keys(sortFunctions).join(', ') + '}';
  console.log(`Usage: ${scriptName} <algorithm> <dataPath>`);
  console.log(`- algorithm: one of ${algorithms}`);
  console.log('- dataPath:  path of test data (json array of integer array test sets)');
}


if (process.argv[1] === import.meta.filename) {
  if (process.argv.length != 4 || !(process.argv[2] in sortFunctions)) {
    printUsage();
    process.exit(1);
  }
  let algorithm = process.argv[2] as keyof typeof sortFunctions;
  let dataPath = process.argv[3];

  try {
    let sortFunction = sortFunctions[algorithm];
    let inputData = await loadData(dataPath);
    let outputData: number[][] = [];
    for (let dataSet of inputData) {
      let sortedData = execute(sortFunction, dataSet)[0];
      outputData.push(sortedData);
    }
    for (let line of serializeData(outputData)) {
      console.log(line);
    }
  }
  catch (error) {
    console.error(`\nError: ${error}`);
    process.exit(1);
  }
}


export { sortFunctions, execute, loadData };
