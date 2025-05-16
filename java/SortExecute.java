import java.io.IOException;
import java.io.PrintStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.Map;
import java.util.LinkedHashMap;
import java.util.function.Consumer;


/**
 * Execute a sorting algorithm with some test data.
 * The test data is a JSON array of integer arrays.
 * The sorting algorithms are implemented in the classes Stage1, Stage2, Stage3, Stage4 and Array.
 */
class SortExecute {

    /**
     * definition of the sorting functions for each stage.
     * initialized in the main method (because literal Map.of() would change order).
     */
    public static final Map<String, Consumer<Element[]>> sortFunctions = getSortFunctions();
    public static Map<String, Consumer<Element[]>> getSortFunctions() {
        LinkedHashMap<String, Consumer<Element[]>> map = new LinkedHashMap<>();
        map.put("stage1", (new Stage1())::sort);
        map.put("stage2", (new Stage2())::sort);
        map.put("stage3", (new Stage3())::sort);
        map.put("stage4", (new Stage4())::sort);
        map.put("jarray", Arrays::sort);
        return map;
    }

    /**
     * result of a sorting operation.
     * contains the sorted data, the comparison count and a string representation of the count.
     */
    class Result {
        int[] outputData;
        int count;
        String countString;

        Result(int[] outputData, int count, String countString) {
            this.outputData = outputData;
            this.count = count;
            this.countString = countString;
        }
    }

    /**
     * execute a sorting function on a test data set.
     * @param sort the sorting function to test.
     * @param inputData the data to sort.
     * @return array of the sorted data, comparison count and comparison count string.
     */
    public Result execute(Consumer<Element[]> sort, int[] inputData) {
        return this.execute(sort, inputData, 3600);
    }

    /**
     * execute a sorting function on a test data set.
     * @param sort the sorting function to test.
     * @param inputData the data to sort.
     * @param limit the maximum number of comparison operations.
     * @return result with the sorted data, comparison count and comparison count string.
     */
    public Result execute(Consumer<Element[]> sort, int[] inputData, int limit) {
        Counter counter = new Counter(limit);
        Element[] elements = new Element[inputData.length];
        for (int i = 0; i < inputData.length; i++) {
            elements[i] = new Element(counter, inputData[i]);
        }
        sort.accept(elements);
        int count = counter.getCount();
        String countString = counter.toString();
        int[] outputData = new int[inputData.length];
        for (int i = 0; i < inputData.length; i++) {
            outputData[i] = elements[i].toInteger();
        }
        return new Result(outputData, count, countString);
    }

    public int[][] loadData(String path) throws IOException {
        String text = Files.readString(Path.of(path));
        return new TestdataParser().parse(text);
    }

    public void saveData(PrintStream stream, int[][] data) throws IOException {
        stream.println("[");
        for (int i = 0; i < data.length; i++) {
            stream.print("  [");
            for (int j = 0; j < data[i].length; j++) {
                if (j > 0) {
                    stream.print(", ");
                }
                stream.print(data[i][j]);
            }
            stream.println(i < data.length - 1 ? "]," : "]");
        }
        stream.println("]");
    }

    static void printUsage() {
        System.out.println("\nExecute a sort algorithm with some test data");
        String scriptName = "SortExecute";
        String algorithms = "{" + String.join(", ", sortFunctions.keySet()) + "}";
        System.out.println("Usage: " + scriptName + " <algorithm> <dataPath>");
        System.out.println("- algorithm: one of " + algorithms);
        System.out.println("- dataPath:  path of test data (json array of integer array test sets)");
    }
    public static void main(String[] args) {
        if (args.length != 2 || !sortFunctions.containsKey(args[0])) {
            printUsage();
            System.exit(1);
        }
        SortExecute sortExecute = new SortExecute();
        String algorithm = args[0];
        String dataPath = args[1];
        try {
            Consumer<Element[]> sortFunction = sortFunctions.get(algorithm);
            int[][] inputData = sortExecute.loadData(dataPath);
            int[][] outputData = new int[inputData.length][];
            for (int i = 0; i < inputData.length; i++) {
                Result result = sortExecute.execute(sortFunction, inputData[i]);
                outputData[i] = result.outputData;
            }
            sortExecute.saveData(System.out, outputData);
        } catch (Exception | Error e) {
            System.err.println("\n" + e.getMessage());
            System.exit(1);
        }
    }

}
