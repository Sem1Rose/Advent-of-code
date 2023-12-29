class Day14
{
    static string txtPath = @"input14.txt";
    static char[][] platform;
    static Dictionary<int, string[]> CycleMap = new();

    static void Start()
    {
        platform = File.ReadLines(txtPath).Select(x => x.ToCharArray()).ToArray();

        Console.WriteLine(PartTwo());
        Console.Read();
    }

    static int PartOne()
    {
        int load = 0;

        platform = RollNorth(platform);

        int index = platform.Length;
        platform.ToList().ForEach(x => load += x.ToList().FindAll(y => y == 'O').Count * index--);

        return load;
    }

    // Top tier documentation || || ||
    //                        \/ \/ \/
    static int PartTwo()
    {
        int load = 0;

        CycleMap.Add(0, ((char[][])platform.Clone()).Select(x => string.Join("", x)).ToArray());

        // After cycle 177, cycles will warp back to 94 and will reset every 84 cycle
        // cycle 178 = cycle 94
        // cycle 179 = cycle 95
        // cycle 261 = cycle 177
        // cycle 262 = cycle 94

        // The monstrosity below calculates which cycle 1 billion will warp to instead of calculating all the way to 1 billion, which would take like forever
        for (int i = 1; i <= (((1e9 - 94) % 84) + 94); i++)
            platform = Cycle(platform);

        int index = platform.Length;
        platform.ToList().ForEach(x => load += x.ToList().FindAll(y => y == 'O').Count * index--);

        return load;
    }

    static char[][] Cycle(char[][] platform) => FlipCW(RollNorth(FlipCW(RollNorth(FlipCW(RollNorth(FlipCW(RollNorth(platform))))))));

    static char[][] RollNorth(char[][] platform)
    {
        for (int y = 1; y < platform.Length; y++)
        {
            for (int x = 0; x < platform[y].Length; x++)
            {
                char character = platform[y][x];
                if (character != 'O')
                    continue;

                char characterAbove = platform[y - 1][x];
                if (characterAbove == '.')
                {
                    platform[y - 1][x] = 'O';
                    platform[y][x] = '.';
                    if (y != 1)
                    {
                        y--;
                        x--;
                    }
                }
            }
        }
        return platform;
    }

    static char[][] FlipCW(char[][] image)
    {
        char[][] flipped = new char[image[0].Length][];

        for (int i = 0; i < image[0].Length; i++)
        {
            char[] arr = new char[image.Length];
            for (int j = image.Length - 1, k = 0; j >= 0; j--, k++)
                arr[k] = image[j][i];

            flipped[i] = arr;
        }

        return flipped;
    }
}