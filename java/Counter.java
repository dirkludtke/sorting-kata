/**
 * counter of the comparison operations which pretends that one comparion takes 1 seconds.
 */
public class Counter {
    int count;
    int limit;

    public Counter() {
        this.count = 0;
        this.limit = 3600;
    }

    public Counter(int limit) {
        this.count = 0;
        this.limit = limit;
    }

    public void increment() {
        this.count++;
        if (this.count >= this.limit) {
            throw new Error("Abort at count " + this.count + ". that corresponds to " + this);
        }
    }

    public int getCount() {
        return count;
    }

    public String toString() {
        int count = this.count;
        if (count < 60) {
            return count != 1 ? count + " seconds" : count + " second";
        }
        count = (count + 30) / 60;
        if (count < 60) {
            return count != 1 ? count + " minutes" : count + " minute";
        }
        count = (count + 30) / 60;
        if (count < 24) {
            return count != 1 ? count + " hours" : count + " hour";
        }
        count = (count + 12) / 24;
        if (count < 366) {
            return count != 1 ? count + " days" : count + " day";
        }
        count = (count + 183) / 366;
        return count != 1 ? count + " years" : count + " year";
    }

}
