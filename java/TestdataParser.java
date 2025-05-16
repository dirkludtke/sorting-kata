import java.text.StringCharacterIterator;
import java.util.ArrayList;

/**
 * as the java standard library does not provide a JSON parser (and we do not want to import
 * external libraries), this class is a simplified parser for two-dimensional int[][] arrays.
 */
public class TestdataParser {

    /**
     * parses a string containing a two-dimensional int[][] array in JSON format. 
     * @param text text to parse.
     * @return parsing result.
     */
    public int[][] parse(String text) {
        StringCharacterIterator it = new StringCharacterIterator(text);
        ArrayList<int[]> testSets = new ArrayList<int[]>();
        findOpen(it);
        if (it.current() != ']') {
            while (true) {
                testSets.add(parseTestset(it));
                if (it.current() == ']') {
                    break;
                }
                if (it.current() != ',') {
                    throw new IllegalArgumentException("Expected , at pos " + it.getIndex());
                }
                it.next();
                skipWhitespace(it);
            }
        }
        it.next();
        skipWhitespace(it);
        if (it.current() != StringCharacterIterator.DONE) {
            throw new IllegalArgumentException("Unexpected character at pos " + it.getIndex());
        }
        return testSets.toArray(new int[testSets.size()][]);
    }

    int[] parseTestset(StringCharacterIterator it) {
        ArrayList<Integer> numbers = new ArrayList<>();
        findOpen(it);
        if (it.current() != ']') {
            while (true) {
                numbers.add(parseNumber(it));
                if (it.current() == ']') {
                    break;
                }
                if (it.current() != ',') {
                    throw new IllegalArgumentException("Expected , at pos " + it.getIndex());
                }
                it.next();
                skipWhitespace(it);
            }
        }
        it.next();
        skipWhitespace(it);
        return numbers.stream().mapToInt(i -> i).toArray();
    }

    int parseNumber(StringCharacterIterator it) {
        skipWhitespace(it);
        char c = it.current();
        if (!Character.isDigit(c)) {
            throw new IllegalArgumentException("Expected number at pos " + it.getIndex());
        }
        int result = Character.digit(c, 10);
        c = it.next();
        while (Character.isDigit(c)) {
            result = result * 10 + Character.digit(c, 10);
            c = it.next();
        }
        skipWhitespace(it);
        return result;
    }

    void findOpen(StringCharacterIterator it) {
        skipWhitespace(it);
        char c = it.current();
        if (c != '[') {
            throw new IllegalArgumentException("Expected [ at pos " + it.getIndex());
        }
        it.next();
        skipWhitespace(it);
    }

    void skipWhitespace(StringCharacterIterator it) {
        char c = it.current();
        while (c == ' ' || c == '\n' || c == '\r' || c == '\t') {
            c = it.next();
        }
    }

}
