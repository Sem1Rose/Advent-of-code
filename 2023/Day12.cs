class Day12
{
    static string txtPath = @"input12.txt";
    static string[] lines;
    static string[] stringRecords;
    static int[][] damagedSpringsSizes;

    // part two would theoretically work, except it would take like a century to run.
    static bool partTwo = false;

    public static void Start()
    {
        lines = File.ReadAllLines(txtPath);
        stringRecords = lines.Select(x => RepeatString(x.Split(" ")[0], "?", partTwo ? 4 : 0)).ToArray();
        damagedSpringsSizes = lines.Select(x => RepeatString(x.Split(" ")[1], ",", partTwo ? 4 : 0).Split(",").Select(int.Parse).ToArray()).ToArray();

        Console.WriteLine(stringRecords[0]);
        damagedSpringsSizes[0].ToList().ForEach(x => Console.Write(x + " "));
        Console.Write('\n');

        Console.WriteLine(GetArrangements());
        Console.Read();
    }

    static int GetArrangements()
    {
        int totalArrangements = 0;

        for (int l = 0; l < stringRecords.Length; l++)
        {
            bool[] recordAsBool = stringRecords[l].Select(x => x == '#').ToArray();
            int[] recordSize = GetSize(recordAsBool);

            List<int> unknownIndecies = new();
            for (int i = 0; i < stringRecords[l].Length; i++)
                if (stringRecords[l][i] == '?')
                    unknownIndecies.Add(i);

            Console.WriteLine((l * 100 / stringRecords.Length) + "%\n" + (1 << unknownIndecies.Count));

            for (int i = 0; i < 1 << unknownIndecies.Count; i++)
            {
                bool[] currentCombination = recordAsBool;

                for (int j = 0; j < unknownIndecies.Count; j++)
                    currentCombination[unknownIndecies[j]] = (i >> j & 1) == 1;

                if (GetSize(currentCombination).SequenceEqual(damagedSpringsSizes[l]))
                    totalArrangements++;
            }
        }

        return totalArrangements;
    }

    static string RepeatString(string str, string seperator, int times) => string.Concat(str, string.Join("", Enumerable.Repeat(seperator + str, times)));

    static int[] GetSize(bool[] record)
    {
        List<int> size = new();

        int batchCount = 0;
        for (int i = 0; i < record.Length; i++)
        {
            if (record[i])
                batchCount++;
            else if (batchCount != 0)
            {
                size.Add(batchCount);
                batchCount = 0;
            }
        }

        if (batchCount != 0)
            size.Add(batchCount);

        return size.ToArray();
    }
}