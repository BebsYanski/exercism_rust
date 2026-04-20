public class Twofer {
    public String twofer(String name) {
        String response = name == null ?  "you" : name;
        return "One for "+ response + ", one for me.";
    }
}
