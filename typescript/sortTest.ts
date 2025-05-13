import { basename, dirname, join} from 'path';
import { sortFunctions, execute, loadData } from './sortExecute.ts';
import type { ListElement } from './Element.ts';


let testDir = join(dirname(import.meta.dirname), 'testdata');

/**
 * test a sorting algorithm with all test data sets until it fails.
 * @param algorithm the sorting algorithm to test.
 */
async function test(algorithm: keyof typeof sortFunctions) {
  console.log(`testing ${algorithm}...`);
  let sort = sortFunctions[algorithm];

  // go through all test data stages
  for (let dataStage = 1; dataStage < 6; dataStage++) {
    // load test data (input and expected output are in separate files)
    let inputName = `input_stage_${dataStage}.json`;
    let outputName = `output_stage_${dataStage}.json`;
    console.log(`  data sets of ${inputName}...`);
    let inputArray = await loadData(join(testDir, inputName));

    // check wheather output data is available. abort if not.
    let outputArray;
    try {
      outputArray = await loadData(join(testDir, outputName));
      if (outputArray.length != inputArray.length) {;
        outputArray.length = 0;
      }
    }
    catch {
    }
    if (outputArray == undefined) {
      throw new Error(`Cannot test ${inputName} because output data is missing (or incorrect)`);
    }    // check all test data sets

    testDatasets(sort, inputArray, outputArray);
  }
}

function testDatasets(
  sort: (elements: ListElement[]) => void,
  inputArray: number[][],
  outputArray: number[][]
) {
  let failCount = 0;
  for (let testId = 0; testId < inputArray.length; testId++) {
    let inputData = inputArray[testId];
    let expected = outputArray[testId];
    let [output, _, countString] = execute(sort, inputData);
    console.log(`    case ${testId + 1} ${listToString(inputData)} in ${countString}`);
    if (!areDatasetsEqual(output, expected)) {
      let outputString = listToString(output, -1);
      let expectedString = listToString(expected, -1);
      console.log(`FAILED.\noutput   ${outputString} !=\nexpected ${expectedString}`);
      failCount++;
    }
  }

  // abort if a test case failed
  if (failCount > 0) {
    let plural = failCount != 1 ? 's' : '';
    throw new Error(`${failCount} test case${plural} failed.`);
  }
}

function areDatasetsEqual(dataset1: number[], dataset2: number[]): boolean {
  if (dataset1.length != dataset2.length) {
    return false;
  }
  for (let i = 0; i < dataset1.length; i++) {
    if (dataset1[i] != dataset2[i]) {
      return false;
    }
  }
  return true;
}

function listToString(data: any[], maxLength: number = 4): string {
  if (maxLength >= 0 && data.length > maxLength) {
    return `[${data.slice(0, maxLength).join(', ')}, ...] (length ${data.length})`;
  }
  return `[${data.join(', ')}]`;
}

function printUsage() {
  console.log('\nTest a sorting algortithm with all test data sets until it fails');
  let scriptName = basename(process.argv[1]);
  let algorithms = '{' + Object.keys(sortFunctions).join(', ') + '}';
  console.log(`Usage: ${scriptName} <algorithm> <dataPath>`);
  console.log(`- algorithm: one of ${algorithms}`);
}


if (process.argv[1] === import.meta.filename) {
  if (process.argv.length != 3 || !(process.argv[2] in sortFunctions)) {
    printUsage();
    process.exit(1);
  }
  let algorithm = process.argv[2] as keyof typeof sortFunctions;

  try {
    await test(algorithm);
  }
  catch (error) {
    console.error(`\n${(error as Error).message}`);
    process.exit(1);
  }
}
