class Day1
{
    readonly static string[][] nums = new string[2][]{
        new string[9] { "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" },
        new string[9] { "1", "2", "3", "4", "5", "6", "7", "8", "9" }
    };

    public static void Start()
    {
        string txtPath = @"input1.txt";
        string[] lines = File.ReadAllLines(txtPath);

        int calibrationValuesSum = 0;

        foreach (var line in lines)
        {
            int[] numbers = new int[line.Length];
            for (int j = 0; j < numbers.Length; j++)
                numbers[j] = -1;

            for (int i = 0; i < 2; i++)
            {
                for (int j = 0; j < 9; j++)
                {
                    int firstIndex = line.IndexOf(nums[i][j]);
                    int lastIndex = line.LastIndexOf(nums[i][j]);

                    if (firstIndex != -1)
                        numbers[firstIndex] = j + 1;
                    if (lastIndex != -1)
                        numbers[lastIndex] = j + 1;
                }
            }

            numbers = numbers.ToList().FindAll(x => x != -1).ToArray();

            calibrationValuesSum += numbers[0] * 10 + numbers[^1];
        }

        Console.WriteLine(calibrationValuesSum);
    }
}