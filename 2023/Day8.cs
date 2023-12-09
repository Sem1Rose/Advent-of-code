class Day8
{
    public static void Start()
    {
        string txtPath = @"input8.txt";
        List<string> lines = File.ReadAllLines(txtPath).ToList();
        int[] directions = new int[lines[0].Length];
        List<int> startIndices = new();

        directions = lines[0].Select(x => x == 'R' ? 1 : 0).ToArray();

        lines = lines.ToArray()[2..].ToList();
        string[] keys = new string[lines.Count];
        for (int i = 0; i < keys.Length; i++)
        {
            string[] split = lines[i].Split(" = ");
            keys[i] = split[0];

            if (split[0].EndsWith('A'))
                startIndices.Add(i);

            lines[i] = split[1][1..^1];
        }

        int[][] map = lines.Select(x => new int[2] { Array.IndexOf(keys, x.Split(", ")[0]), Array.IndexOf(keys, x.Split(", ")[1]) }).ToArray();
        lines.Clear();

        ulong steps = 0;
        ulong dirLength = (ulong)directions.Length;

        ulong leastCommonMult = 1;
        for (int i = 0; i < startIndices.Count; i++)
        {
            steps = 0;
            while (!keys[startIndices[i]].EndsWith('Z'))
                startIndices[i] = map[startIndices[i]][directions[steps++ % dirLength]];

            leastCommonMult = LeastCommonMultiple(leastCommonMult, steps);
        }

        Console.WriteLine("\n" + leastCommonMult);

        Console.Read();
    }

    // yeeted from stackoverflow
    static ulong GreatestCommonFactor(ulong a, ulong b)
    {
        while (b != 0)
        {
            ulong temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }
    static ulong LeastCommonMultiple(ulong a, ulong b)
    {
        return a / GreatestCommonFactor(a, b) * b;
    }
}