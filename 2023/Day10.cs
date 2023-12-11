class Day10
{
    static string txtPath = @"input10.txt";
    static char[][] maze;
    static int[] index;
    static int[] startIndex = new int[2];
    static int startVal = -1;
    public static void Start()
    {
        maze = File.ReadAllLines(txtPath).Select(x => x.ToCharArray()).ToArray();

        for (int i = 0; i < maze.Length; i++)
            for (int j = 0; j < maze[i].Length; j++)
                if (maze[i][j] == 'S')
                    startIndex = new int[2] { i, j };

        index = (int[])startIndex.Clone();

        Console.Read();
    }

    static int PartOne()
    {
        List<int> distsCW = new();
        List<int> distsCCW = new();

        int lastDir = -1;
        int dists = -1;
        while (true)
        {
            dists++;
            distsCW.Add(dists);

            char cell = maze[index[0]][index[1]];

            if (lastDir != 2)
            {
                if (index[0] > 0 && (cell == 'S' || cell == '|' || cell == 'L' || cell == 'J')) // UP
                {
                    char next = maze[index[0] - 1][index[1]];
                    if (next == 'S')
                        break;

                    if (next == '|' || next == '7' || next == 'F')
                    {
                        index[0]--;
                        lastDir = 0;
                        continue;
                    }
                }
            }

            if (lastDir != 3)
            {
                if (index[1] < maze[index[0]].Length - 1 && (cell == 'S' || cell == '-' || cell == 'L' || cell == 'F')) // RIGHT
                {
                    char next = maze[index[0]][index[1] + 1];
                    if (next == 'S')
                        break;

                    if (next == '-' || next == '7' || next == 'J')
                    {
                        index[1]++;
                        lastDir = 1;
                        continue;
                    }
                }
            }

            if (lastDir != 0)
            {
                if (index[0] < maze.Length - 1 && (cell == 'S' || cell == '|' || cell == '7' || cell == 'F')) // DOWN
                {
                    char next = maze[index[0] + 1][index[1]];
                    if (next == 'S')
                        break;

                    if (next == '|' || next == 'J' || next == 'L')
                    {
                        index[0]++;
                        lastDir = 2;
                        continue;
                    }
                }
            }

            if (lastDir != 1)
            {
                if (index[1] > 0 && (cell == 'S' || cell == '-' || cell == 'J' || cell == '7')) // LEFT
                {
                    char next = maze[index[0]][index[1] - 1];
                    if (next == 'S')
                        break;

                    if (next == '-' || next == 'L' || next == 'F')
                    {
                        index[1]--;
                        lastDir = 3;
                        continue;
                    }
                }
            }
        }

        lastDir = -1;
        dists = -1;
        index = (int[])startIndex.Clone();
        while (true)
        {
            dists++;
            distsCCW.Add(dists);

            char cell = maze[index[0]][index[1]];

            if (lastDir != 0)
            {
                if (index[0] < maze.Length - 1 && (cell == 'S' || cell == '|' || cell == '7' || cell == 'F')) // DOWN
                {
                    char next = maze[index[0] + 1][index[1]];
                    if (next == 'S')
                        break;

                    if (next == '|' || next == 'J' || next == 'L')
                    {
                        index[0]++;
                        lastDir = 2;
                        continue;
                    }
                }
            }

            if (lastDir != 1)
            {
                if (index[1] > 0 && (cell == 'S' || cell == '-' || cell == 'J' || cell == '7')) // LEFT
                {
                    char next = maze[index[0]][index[1] - 1];
                    if (next == 'S')
                        break;

                    if (next == '-' || next == 'L' || next == 'F')
                    {
                        index[1]--;
                        lastDir = 3;
                        continue;
                    }
                }
            }

            if (lastDir != 2)
            {
                if (index[0] > 0 && (cell == 'S' || cell == '|' || cell == 'L' || cell == 'J')) // UP
                {
                    char next = maze[index[0] - 1][index[1]];
                    if (next == 'S')
                        break;

                    if (next == '|' || next == '7' || next == 'F')
                    {
                        index[0]--;
                        lastDir = 0;
                        continue;
                    }
                }
            }

            if (lastDir != 3)
            {
                if (index[1] < maze[index[0]].Length - 1 && (cell == 'S' || cell == '-' || cell == 'L' || cell == 'F')) // RIGHT
                {
                    char next = maze[index[0]][index[1] + 1];
                    if (next == 'S')
                        break;

                    if (next == '-' || next == '7' || next == 'J')
                    {
                        index[1]++;
                        lastDir = 1;
                        continue;
                    }
                }
            }
            break;
        }
        distsCCW.RemoveAt(0);
        distsCW.RemoveAt(0);

        distsCCW.Reverse();

        int max = -1;
        for (int i = 0; i < distsCW.Count; i++)
            max = Math.Max(max, Math.Min(distsCW[i], distsCCW[i]));

        return max;
    }

    static int PartTwo()
    {
        Dictionary<int, int> path = new();

        bool[] startBoundary = new bool[4];

        if (index[0] > 0)
            if (maze[index[0] - 1][index[1]] == '|' || maze[index[0] - 1][index[1]] == 'F' || maze[index[0] - 1][index[1]] == '7')
                startBoundary[0] = true;
        if (index[0] < maze.Length - 1)
            if (maze[index[0] + 1][index[1]] == '|' || maze[index[0] + 1][index[1]] == 'J' || maze[index[0] + 1][index[1]] == 'L')
                startBoundary[2] = true;
        if (index[1] > 0)
            if (maze[index[0]][index[1] - 1] == '-' || maze[index[0]][index[1] - 1] == 'F' || maze[index[0]][index[1] - 1] == 'L')
                startBoundary[3] = true;
        if (index[1] < maze[index[0]].Length - 1)
            if (maze[index[0]][index[1] + 1] == '-' || maze[index[0]][index[1] + 1] == 'J' || maze[index[0]][index[1] + 1] == '7')
                startBoundary[1] = true;

        if (startBoundary[1] && startBoundary[3])
            startVal = -1;
        if (startBoundary[0] && startBoundary[2])
            startVal = 0;
        if (startBoundary[0] && startBoundary[1])
            startVal = 1;
        if (startBoundary[3] && startBoundary[0])
            startVal = 1;
        if (startBoundary[1] && startBoundary[2])
            startVal = 2;
        if (startBoundary[2] && startBoundary[3])
            startVal = 2;

        int lastDir = -1;
        while (true)
        {
            char cell = maze[index[0]][index[1]];
            int cellVal = cell == '|' ? 0 : cell == 'L' || cell == 'J' ? 1 : cell == 'F' || cell == '7' ? 2 : cell == 'S' ? startVal : -1;


            path.Add(index[0] * maze[0].Length + index[1], cellVal);

            if (lastDir != 2)
            {
                if (index[0] > 0 && (cell == 'S' || cell == '|' || cell == 'L' || cell == 'J')) // UP
                {
                    char next = maze[index[0] - 1][index[1]];
                    if (next == 'S')
                        break;

                    if (next == '|' || next == '7' || next == 'F')
                    {
                        index[0]--;
                        lastDir = 0;
                        continue;
                    }
                }
            }

            if (lastDir != 3)
            {
                if (index[1] < maze[index[0]].Length - 1 && (cell == 'S' || cell == '-' || cell == 'L' || cell == 'F')) // RIGHT
                {
                    char next = maze[index[0]][index[1] + 1];
                    if (next == 'S')
                        break;

                    if (next == '-' || next == '7' || next == 'J')
                    {
                        index[1]++;
                        lastDir = 1;
                        continue;
                    }
                }
            }

            if (lastDir != 0)
            {
                if (index[0] < maze.Length - 1 && (cell == 'S' || cell == '|' || cell == '7' || cell == 'F')) // DOWN
                {
                    char next = maze[index[0] + 1][index[1]];
                    if (next == 'S')
                        break;

                    if (next == '|' || next == 'J' || next == 'L')
                    {
                        index[0]++;
                        lastDir = 2;
                        continue;
                    }
                }
            }

            if (lastDir != 1)
            {
                if (index[1] > 0 && (cell == 'S' || cell == '-' || cell == 'J' || cell == '7')) // LEFT
                {
                    char next = maze[index[0]][index[1] - 1];
                    if (next == 'S')
                        break;

                    if (next == '-' || next == 'L' || next == 'F')
                    {
                        index[1]--;
                        lastDir = 3;
                        continue;
                    }
                }
            }
        }

        int numEnclosedTiles = 0;
        for (int i = 0; i < maze.Length; i++)
        {
            for (int j = 0; j < maze[i].Length; j++)
            {
                int indx = i * maze[i].Length + j;
                if (path.ContainsKey(indx))
                    continue;

                int lastIntersection = -1;
                int numPathIntersections = 0;
                for (int k = j - 1; k >= 0; k--)
                {
                    if (path.ContainsKey(i * maze[i].Length + k) && path[i * maze[i].Length + k] >= 0)
                    {
                        if (lastIntersection == 1 && path[i * maze[i].Length + k] == 2 || lastIntersection == 2 && path[i * maze[i].Length + k] == 1)
                        {
                            lastIntersection = -1;
                            continue;
                        }
                        if (lastIntersection == path[i * maze[i].Length + k])
                        {
                            lastIntersection = -1;
                            numPathIntersections--;
                            continue;
                        }
                        numPathIntersections++;
                        lastIntersection = path[i * maze[i].Length + k] > 0 ? path[i * maze[i].Length + k] : -1;
                    }
                }

                numEnclosedTiles += (numPathIntersections % 2) == 1 ? 1 : 0;
            }
        }

        return numEnclosedTiles;
    }
}