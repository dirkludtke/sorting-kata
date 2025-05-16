/**
 * a wrapper around integers which counts the number of comparison and value accesses.
 * - it also ensures that integers are in a range from 0 to 65535 (uint16).
 */
public class Element implements Comparable<Element> {
    Counter counter;
    int value;

    public Element(Counter counter, int value) {
        if (value < 0 || value >= Math.pow(2, 16)) {
            throw new Error("Element value must be uint 16 (0-65535), not " + value);
        }
        this.counter = counter;
        this.value = value;
    }

    @Override
    public int compareTo(Element other) {
        return Integer.signum(this.value - other.toInteger());
    }

    public boolean lessThan(Element other) {
        return this.compareTo(other) < 0;
    }

    public boolean lessEqual(Element other) {
        return this.compareTo(other) <= 0;
    }

    public boolean equal(Element other) {
        return this.compareTo(other) == 0;
    }

    public boolean greaterEqual(Element other) {
        return this.compareTo(other) >= 0;
    }

    public boolean greaterThan(Element other) {
        return this.compareTo(other) > 0;
    }

    public int toInteger() {
        this.counter.increment();
        return this.value;
    }

    public String toString() {
        return Integer.toString(this.value);
    }

}
