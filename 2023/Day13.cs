class Day13
{
    static string txtPath = @"input13.txt";
    static string[][] maps;

    static bool partTwo = true;

    static void Start()
    {
        maps = File.ReadAllText(txtPath).Split("\n\r\n").Select(x => x.Split("\r\n")).ToArray();

        Console.WriteLine(Solution());
        Console.ReadLine();
    }

    static int Solution()
    {
        int numVerticalLines = 0;
        int numHorizontalLines = 0;

        for (int i = 0; i < maps.Length; i++)
        {
            string[] currMap = maps[i];

            for (int j = 0; j < 4; j++)
            {
                currMap = Flip(currMap);
                int reflectionPoint = partTwo ? FindReflectionPointPartTwo(currMap) : FindReflectionPoint(currMap);
                if (reflectionPoint != -1)
                    if (j == 0)
                        numVerticalLines += currMap.Length - reflectionPoint - 1;
                    else if (j == 1)
                        numHorizontalLines += currMap.Length - reflectionPoint - 1;
                    else if (j == 2)
                        numVerticalLines += reflectionPoint + 1;
                    else if (j == 3)
                        numHorizontalLines += reflectionPoint + 1;
            }
        }
        return numHorizontalLines * 100 + numVerticalLines;
    }

    static int FindReflectionPoint(string[] map)
    {
        int x = 1;
        if ((map.Length & 1) == 1)
            x++;

        for (int i = map.Length - x; i >= 1; i -= 2)
        {
            if (map[0] == map[i])
            {
                bool equal = true;
                for (int j = 1; j <= (int)Math.Floor(i / 2f); j++)
                {
                    if (map[j] != map[i - j])
                    {
                        equal = false;
                        break;
                    }
                }

                if (equal)
                    return (int)Math.Floor(i / 2f);
            }
        }
        return -1;
    }

    static int FindReflectionPointPartTwo(string[] map)
    {
        int x = 1;
        if ((map.Length & 1) == 1)
            x++;

        for (int i = map.Length - x; i >= 1; i -= 2)
        {
            bool smudgeInParent = OneCharDifference(map[0], map[i]);
            if (map[0] == map[i] || smudgeInParent)
            {
                bool equal = true;
                bool smudgeInChild = false;
                for (int j = 1; j <= (int)Math.Floor(i / 2f); j++)
                {
                    if (map[j] != map[i - j])
                    {
                        if (OneCharDifference(map[j], map[i - j]))
                        {
                            smudgeInChild = true;
                            continue;
                        }
                        equal = false;
                        break;
                    }
                }

                if (equal && (smudgeInParent || smudgeInChild))
                    return (int)Math.Floor(i / 2f);
            }
        }
        return -1;
    }

    static string[] Flip(string[] image)
    {
        string[] flipped = new string[image[0].Length];

        for (int i = 0; i < image[0].Length; i++)
        {
            char[] arr = new char[image.Length];
            for (int j = 0; j < image.Length; j++)
                arr[j] = image[j].ToCharArray()[i];

            flipped[i] = string.Join("", arr);
        }

        flipped = flipped.Reverse().ToArray();
        return flipped;
    }

    static bool OneCharDifference(string first, string second)
    {
        int numDifferences = 0;
        for (int i = 0; i < first.Length; i++)
        {
            if (first[i] != second[i])
                numDifferences++;
            if (numDifferences > 1)
                return false;
        }

        return numDifferences == 1;
    }
}