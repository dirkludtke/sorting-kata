import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.function.Consumer;
import java.util.stream.IntStream;


/**
 * Test a sorting algorithm with all test data sets until it fails.
 */
public class SortTest {

    /**
     * test a sorting algorithm with all test data sets until it fails.
     * @param algorithm the sorting algorithm to test.
     */
    public void test(String algorithm) throws URISyntaxException, IOException {
        System.out.println("testing " + algorithm + "...");
        SortExecute sortExecute = new SortExecute();
        Consumer<Element[]> sort = SortExecute.sortFunctions.get(algorithm);

        Path classPath = Path.of(SortTest.class.getProtectionDomain().getCodeSource().getLocation().toURI());
        Path testDir = classPath.getParent().resolve("testdata");

        for (int dataStage = 1; dataStage < 6; dataStage++) {
            // load test data (input and expected output are in separate files)
            String inputName = "input_stage_" + dataStage + ".json";
            String outputName = "output_stage_" + dataStage + ".json";
            System.out.println("  data sets of " + inputName + "...");
            int[][] inputArray = sortExecute.loadData(testDir.resolve(inputName).toString());

            // check whether output data is available. abort if not.
            int[][] outputArray = null;
            try {
                outputArray = sortExecute.loadData(testDir.resolve(outputName).toString());
                if (outputArray.length != inputArray.length) {
                    outputArray = null;
                }
            } catch (Exception e) {
            }
            if (outputArray == null) {
                throw new RuntimeException("Cannot test " + inputName + " because output data is missing (or incorrect)");
            }

            // check all test data sets
            testDatasets(sortExecute, sort, inputArray, outputArray);
        }
    }

    void testDatasets(
        SortExecute sortExecute, Consumer<Element[]> sort, int[][] inputArray, int[][] outputArray
    ) {
        int failCount = 0;
        for (int testId = 0; testId < inputArray.length; testId++) {
            int[] inputData = inputArray[testId];
            int[] expected = outputArray[testId];
            SortExecute.Result result = sortExecute.execute(sort, inputData);
            System.out.println("    case " + (testId + 1) + " " + listToString(inputData) + " in " + result.countString);
            if (!Arrays.equals(result.outputData, expected)) {
                String outputString = listToString(result.outputData, -1);
                String expectedString = listToString(expected, -1);
                System.out.println("FAILED.\noutput   " + outputString + " !=\nexpected " + expectedString);
                failCount++;
            }
        }

        // abort if a test case failed
        if (failCount > 0) {
            String plural = failCount != 1 ? "s" : "";
            throw new RuntimeException("FAILED " + failCount + " test case" + plural + "failed.");
        }
    }

    String listToString(int[] data) {
        return listToString(data, 4);
    }

    String listToString(int[] data, int maxLength) {
        if (maxLength >= 0 && data.length > maxLength) {
            String commaString = toCommaString(Arrays.copyOf(data, maxLength));
            return "[" + commaString + ", ...] (length " + data.length + ")";
        }
        return "[" + toCommaString(data) + "]";
    }

    String toCommaString(int[] data) {
        return String.join(
            ", ",
            IntStream.of(data).mapToObj(Integer::toString).toArray(String[]::new)
        );
    }

    static void printUsage() {
        System.out.println("\nTest a sorting algortithm with all test data sets until it fails");
        String scriptName = "SortTest";
        String algorithms = "{" + String.join(", ", SortExecute.sortFunctions.keySet()) + "}";
        System.out.println("Usage: " + scriptName + " <algorithm> <dataPath>");
        System.out.println("- algorithm: one of " + algorithms);
    }

    public static void main(String[] args) {
        if (args.length != 1 || !SortExecute.sortFunctions.containsKey(args[0])) {
            printUsage();
            System.exit(1);
        }
        SortTest sortTest = new SortTest();
        String algorithm = args[0];
        try {
            sortTest.test(algorithm);
        } catch (Exception | Error e) {
            System.err.println("\n" + e.getMessage());
            System.exit(1);
        }
    }

}
