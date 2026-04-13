
public class Lasagna {
    private static final int PREPARATION_TIME_PER_LAYER = 2;
    private static final int EXPECTED_MINUTES_IN_OVEN = 40;


    public int expectedMinutesInOven(){
        return EXPECTED_MINUTES_IN_OVEN;
    }

public int remainingMinutesInOven(int timeSpent){
    return expectedMinutesInOven() - timeSpent;
}
    public int preparationTimeInMinutes(int numberOfLayers){
        return PREPARATION_TIME_PER_LAYER* numberOfLayers;
    }

    public int totalTimeInMinutes(int numOfLayers,int numOfMinutes){
        return preparationTimeInMinutes(numOfLayers) + numOfMinutes;
    }
}
