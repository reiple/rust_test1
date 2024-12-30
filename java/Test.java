import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Test {
    public static void main(String[] args) {
        
        String dateTimePattern = "\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}";
        Pattern pattern = Pattern.compile(dateTimePattern);

        System.out.println("CREATE STRINGS...");
        List<String> testStrings = new ArrayList<>();
        for (int i = 0; i < 100_000_100; i++) {
            if (i % 2 == 0) {
                testStrings.add(String.format("Event at 2024-12-26 15:%02d:45", i % 60));
            } else {
                testStrings.add(String.format("Random text with no date %d", i));
            }
        }

        System.out.println("START!");
        long startTime = System.currentTimeMillis();

        int matchCount = 0;
        for (String text : testStrings) {
            Matcher matcher = pattern.matcher(text);
            if (matcher.find()) {
                matchCount++;
            }
			
			if(matchCount == 50) {
				startTime = System.currentTimeMillis();
			}
        }

        long duration = System.currentTimeMillis() - startTime;
		System.out.println("DONE!");
        
        System.out.println("Checked " + testStrings.size() + " strings.");
        System.out.println("Found " + matchCount + " matches.");
        System.out.println("Time taken: " + duration + " ms");
    }
}