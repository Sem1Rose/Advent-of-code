using System.ComponentModel;

class Day11
{
    static string txtPath = @"input11.txt";
    static string[] lines;


    public static void Main()
    {
        lines = File.ReadAllLines(txtPath);

        Console.WriteLine(PartTwo());
        Console.Read();
    }

    static int PartOne()
    {
        int temp = 0;
        List<int[]> image = Flip(lines.ToList().Select(x => x.ToCharArray().Select(x => x == '#' ? ++temp : -1).ToArray()).ToList());
        int[][] galaxiesIndicies = new int[temp][];

        image = Flip(Flip(DuplicateEmptyRows(Flip(DuplicateEmptyRows(image)))));

        temp = 0;
        for (int i = 0; i < image.Count; i++)
            for (int j = 0; j < image[i].Length; j++)
                if (image[i][j] > 0)
                    galaxiesIndicies[temp++] = new int[2] { i, j };

        temp = 0;
        int numCombinations = nC2(galaxiesIndicies.Length);
        int[][] combinations = new int[numCombinations][];
        for (int i = 0; i < galaxiesIndicies.Length - 1; i++)
            for (int j = i + 1; j < galaxiesIndicies.Length; j++)
                combinations[temp++] = new int[2] { i, j };

        int[] distances = new int[numCombinations];
        for (int i = 0; i < combinations.Length; i++)
        {
            int[] firstCoords = galaxiesIndicies[combinations[i][0]];
            int[] secondCoords = galaxiesIndicies[combinations[i][1]];

            distances[i] = Math.Abs(secondCoords[0] - firstCoords[0]) + Math.Abs(secondCoords[1] - firstCoords[1]);
        }

        int sum = 0;
        distances.ToList().ForEach(x => sum += x);

        return sum;
    }

    static long PartTwo()
    {
        int temp = 0;
        List<int[]> image = lines.ToList().Select(x => x.ToCharArray().Select(x => x == '#' ? ++temp : -1).ToArray()).ToList();
        int[][] galaxiesIndicies = new int[temp][];

        List<int> duplicateRows = GetDuplicatedRows(image);                         // -
        List<int> duplicateColumns = GetDuplicatedRows(Flip(Flip(Flip(image))));    // |

        temp = 0;
        for (int i = 0; i < image.Count; i++)
            for (int j = 0; j < image[i].Length; j++)
                if (image[i][j] > 0)
                    galaxiesIndicies[temp++] = new int[2] { i, j };

        temp = 0;
        int numCombinations = nC2(galaxiesIndicies.Length);
        int[][] combinations = new int[numCombinations][];
        for (int i = 0; i < galaxiesIndicies.Length - 1; i++)
            for (int j = i + 1; j < galaxiesIndicies.Length; j++)
                combinations[temp++] = new int[2] { i, j };

        long[] distances = new long[numCombinations];
        for (int i = 0; i < combinations.Length; i++)
        {
            int[] firstCoords = galaxiesIndicies[combinations[i][0]];
            int[] secondCoords = galaxiesIndicies[combinations[i][1]];

            int numDupRows = duplicateRows.FindAll(x => x < Math.Max(firstCoords[0], secondCoords[0]) && x > Math.Min(firstCoords[0], secondCoords[0])).Count;
            int numDupColumns = duplicateColumns.FindAll(x => x < Math.Max(firstCoords[1], secondCoords[1]) && x > Math.Min(firstCoords[1], secondCoords[1])).Count;

            distances[i] = Math.Abs(secondCoords[0] - firstCoords[0]) + (long)(1E6 - 1) * numDupRows + Math.Abs(secondCoords[1] - firstCoords[1]) + (long)(1E6 - 1) * numDupColumns;
        }

        long sum = 0;
        distances.ToList().ForEach(x => sum += x);

        return sum;
    }

    static List<int[]> Flip(List<int[]> image)
    {
        List<int[]> processed = new();

        for (int i = 0; i < image[0].Length; i++)
        {
            int[] arr = new int[image.Count];
            for (int j = 0; j < image.Count; j++)
                arr[j] = image[j][i];

            processed.Add(arr);
        }

        processed.Reverse();
        return processed;
    }

    static List<int[]> DuplicateEmptyRows(List<int[]> image)
    {
        for (int i = 0; i < image.Count; i++)
        {
            bool empty = true;
            for (int j = 0; j < image[i].Length; j++)
            {
                if (image[i][j] != -1)
                {
                    empty = false;
                    break;
                }
            }

            if (empty)
            {
                image.Insert(i, image[i]);
                i++;
            }
        }
        return image;
    }

    static List<int> GetDuplicatedRows(List<int[]> image)
    {
        List<int> dupRowsInds = new();
        for (int i = 0; i < image.Count; i++)
        {
            bool empty = true;
            for (int j = 0; j < image[i].Length; j++)
            {
                if (image[i][j] != -1)
                {
                    empty = false;
                    break;
                }
            }

            if (empty)
                dupRowsInds.Add(i);
        }
        return dupRowsInds;
    }

    // STACKOVERFLOWWWWWWW
    static int nC2(int n)
    {
        int result = 1;
        for (int i = n; i > n - 2; i--)
            result *= i;
        return result / 2;
    }
}