class Day9
{
    public static void Start()
    {
        string txtPath = @"input9.txt";
        int[][] lines = File.ReadAllLines(txtPath).Select(x => x.Split(" ").Select(x => int.Parse(x)).ToArray()).ToArray();
        long sum = 0;

        foreach (var line in lines)
        {
            List<int[]> deltas = new()
            {
                line,
                CalculateDeltas(line)
            };

            while (deltas[^1].Select(x => x != 0).Any(x => x == true))
                deltas.Add(CalculateDeltas(deltas[^1]));

            deltas = deltas.Select(x => x.Reverse().ToArray()).ToList();

            var finalNum = 0;
            for (int i = deltas.Count - 2; i >= 0; i--)
                finalNum = deltas[i][^1] - finalNum;

            sum += finalNum;
            Console.WriteLine(finalNum);
        }
        Console.WriteLine(sum);
        Console.Read();
    }

    static int[] CalculateDeltas(int[] input)
    {
        int[] deltas = new int[input.Length - 1];
        for (int i = 1; i < input.Length; i++)
            deltas[i - 1] = input[i] - input[i - 1];

        return deltas;
    }
}