// Kallaf

class Day4
{
    public static void Start()
    {
        string txtPath = @"input4.txt";
        string[] cards = File.ReadAllLines(txtPath);

        // first part solution
        /*Dictionary<int, int> cardVals = new();
        int sum = 0;

        for (int i = 0; i < cards.Length; i++)
        {
            cardVals.Add(i, 0);

            string nums = cards[i].Split(": ")[1];
            string[] winningNums = nums.Split(" | ")[0].Split(" ");
            string[] cardNums = nums.Split(" | ")[1].Split(" ");

            for (int j = 0; j < cardNums.Length; j++)
            {
                string num = cardNums[j];
                if (string.IsNullOrWhiteSpace(num))
                    continue;

                if (winningNums.Contains(num))
                    if (cardVals[i] == 0)
                        cardVals[i]++;
                    else
                        cardVals[i] *= 2;
            }
        }

        cardVals.Select(x => x.Value).ToList().ForEach(x => sum += x);*/
        int[] cardInstances = new int[cards.Length];
        int sum = 0;

        for (int i = 0; i < cards.Length; i++)
        {
            cardInstances[i]++;
            for (int k = 0; k < cardInstances[i]; k++)
            {
                string nums = cards[i].Split(": ")[1];
                string[] winningNums = nums.Split(" | ")[0].Split(" ");
                string[] cardNums = nums.Split(" | ")[1].Split(" ");
                int matchingNums = 0;

                for (int j = 0; j < cardNums.Length; j++)
                {
                    string num = cardNums[j];
                    if (string.IsNullOrWhiteSpace(num))
                        continue;

                    if (winningNums.Contains(num))
                        matchingNums++;
                }

                for (int j = 0; j < matchingNums; j++)
                    cardInstances[i + j + 1]++;
            }
            Console.WriteLine($"{(float)(i + 1) * 100 / cards.Length}%");
        }

        cardInstances.ToList().ForEach(x => sum += x);

        Console.WriteLine();
        Console.WriteLine(sum);
    }
}