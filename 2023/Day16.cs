using System.Numerics;

class Day16
{
    static readonly string txtPath = @"input16.txt";
    static char[][] grid;
    static char[][] energizedTiles;
    static List<(Vector2 pos, int dirs)> cashedTilesRays;

    static void Start()
    {
        grid = File.ReadAllLines(txtPath).Select(x => x.ToCharArray()).ToArray();
        cashedTilesRays = new();

        Console.WriteLine(PartTwo());
        Console.Read();
    }

    static int PartOne()
    {
        energizedTiles = grid.Select(x => x.Select(y => '.').ToArray()).ToArray();
        cashedTilesRays.Clear();
        SimulateRay2(new Vector2(-1, 0), new Vector2(1, 0));

        return energizedTiles.SelectMany(x => x.ToList().FindAll(y => y == '#').ToArray()).ToList().Count;
    }

    static int PartTwo()
    {
        List<int> counts = new();

        for (int i = 0; i < 2; i++)
        {
            for (int j = 0; j < grid.Length; j++)
            {
                Console.WriteLine(i + " " + j);
                energizedTiles = grid.Select(x => x.Select(y => '.').ToArray()).ToArray();
                cashedTilesRays.Clear();
                SimulateRay2(new Vector2(i == 0 ? -1 : grid[0].Length, j), new Vector2(i == 0 ? 1 : -1, 0));
                counts.Add(energizedTiles.SelectMany(x => x.ToList().FindAll(y => y == '#').ToArray()).ToList().Count);
                // Console.WriteLine($"({(i == 0 ? -1 : grid[0].Length)}, {j}), ({(i == 0 ? 1 : -1)}, 0)");
            }
        }
        Console.WriteLine();
        for (int i = 0; i < 2; i++)
        {
            for (int j = 0; j < grid[0].Length; j++)
            {
                Console.WriteLine(i + " " + j);
                energizedTiles = grid.Select(x => x.Select(y => '.').ToArray()).ToArray();
                cashedTilesRays.Clear();
                SimulateRay2(new Vector2(j, i == 0 ? -1 : grid.Length), new Vector2(0, i == 0 ? 1 : -1));
                counts.Add(energizedTiles.SelectMany(x => x.ToList().FindAll(y => y == '#').ToArray()).ToList().Count);
                // Console.WriteLine($"({j}, {(i == 0 ? -1 : grid.Length)}), (0, {(i == 0 ? 1 : -1)})");
            }
        }

        return counts.Max();
    }

    //                         /\ /\ /\
    // Iteration, UPUPUPUPUP   || || ||
    static void SimulateRay2(Vector2 startPos, Vector2 startDir)
    {
        List<(Vector2 pos, Vector2 dir)> positionDirections = new() { (startPos, startDir) };

        while (true)
        {
            for (int i = 0; i < positionDirections.Count; i++)
            {
                Vector2 pos, dir;
                (pos, dir) = positionDirections[i];

                positionDirections.RemoveAt(i);

                if (cashedTilesRays.Any(x => x.pos == pos))
                {
                    int index = cashedTilesRays.IndexOf(cashedTilesRays.First(x => x.pos == pos));
                    if ((cashedTilesRays[index].dirs & CompressDir(dir)) != 0)
                    {
                        i--;
                        continue;
                    }
                    else
                        cashedTilesRays[index] = (pos, cashedTilesRays[index].dirs | CompressDir(dir));
                }
                else
                    cashedTilesRays.Add((pos, CompressDir(dir)));

                Vector2 newPos = pos + dir;

                if (newPos.X >= grid[0].Length || newPos.X < 0)
                {
                    i--;
                    continue;
                }
                if (newPos.Y >= grid.Length || newPos.Y < 0)
                {
                    i--;
                    continue;
                }

                energizedTiles[(int)newPos.Y][(int)newPos.X] = '#';

                char nextBlock = grid[(int)newPos.Y][(int)newPos.X];
                switch (nextBlock)
                {
                    case '.':
                        positionDirections.Add((newPos, dir));
                        break;
                    case '|':
                        if (dir.X == 0)
                            positionDirections.Add((newPos, dir));
                        else
                        {
                            positionDirections.Add((newPos, new Vector2(0, 1)));
                            positionDirections.Add((newPos, new Vector2(0, -1)));
                        }
                        break;
                    case '-':
                        if (dir.Y == 0)
                            positionDirections.Add((newPos, dir));
                        else
                        {
                            positionDirections.Add((newPos, new Vector2(1, 0)));
                            positionDirections.Add((newPos, new Vector2(-1, 0)));
                        }
                        break;
                    case '/':
                        if (dir.X > 0)
                            positionDirections.Add((newPos, new Vector2(0, -1)));
                        else if (dir.X < 0)
                            positionDirections.Add((newPos, new Vector2(0, 1)));
                        else if (dir.Y > 0)
                            positionDirections.Add((newPos, new Vector2(-1, 0)));
                        else if (dir.Y < 0)
                            positionDirections.Add((newPos, new Vector2(1, 0)));
                        break;
                    case '\\':
                        if (dir.X < 0)
                            positionDirections.Add((newPos, new Vector2(0, -1)));
                        else if (dir.X > 0)
                            positionDirections.Add((newPos, new Vector2(0, 1)));
                        else if (dir.Y < 0)
                            positionDirections.Add((newPos, new Vector2(-1, 0)));
                        else if (dir.Y > 0)
                            positionDirections.Add((newPos, new Vector2(1, 0)));
                        break;
                    default:
                        break;
                }
            }

            if (positionDirections.Count == 0)
                break;
        }
    }

    // Recursion, Didn't work
    static void SimulateRay(Vector2 pos, Vector2 dir)
    {
        if (cashedTilesRays.Any(x => x.pos == pos))
        {
            int index = cashedTilesRays.IndexOf(cashedTilesRays.First(x => x.pos == pos));
            if ((cashedTilesRays[index].dirs & CompressDir(dir)) != 0)
                return;
            else
                cashedTilesRays[index] = (pos, cashedTilesRays[index].dirs | CompressDir(dir));
        }
        else
            cashedTilesRays.Add((pos, CompressDir(dir)));

        Vector2 newPos = pos + dir;

        if (newPos.X >= grid[0].Length || newPos.X < 0)
            return;
        if (newPos.Y >= grid.Length || newPos.Y < 0)
            return;

        energizedTiles[(int)newPos.Y][(int)newPos.X] = '#';

        char nextBlock = grid[(int)newPos.Y][(int)newPos.X];
        switch (nextBlock)
        {
            case '.':
                SimulateRay(newPos, dir);
                break;
            case '|':
                if (dir.X == 0)
                    SimulateRay(newPos, dir);
                else
                {
                    SimulateRay(newPos, new Vector2(0, 1));
                    SimulateRay(newPos, new Vector2(0, -1));
                }
                break;
            case '-':
                if (dir.Y == 0)
                    SimulateRay(newPos, dir);
                else
                {
                    SimulateRay(newPos, new Vector2(1, 0));
                    SimulateRay(newPos, new Vector2(-1, 0));
                }
                break;
            case '/':
                if (dir.X > 0)
                    SimulateRay(newPos, new Vector2(0, -1));
                else if (dir.X < 0)
                    SimulateRay(newPos, new Vector2(0, 1));
                else if (dir.Y > 0)
                    SimulateRay(newPos, new Vector2(-1, 0));
                else if (dir.Y < 0)
                    SimulateRay(newPos, new Vector2(1, 0));
                break;
            case '\\':
                if (dir.X < 0)
                    SimulateRay(newPos, new Vector2(0, -1));
                else if (dir.X > 0)
                    SimulateRay(newPos, new Vector2(0, 1));
                else if (dir.Y < 0)
                    SimulateRay(newPos, new Vector2(-1, 0));
                else if (dir.Y > 0)
                    SimulateRay(newPos, new Vector2(1, 0));
                break;
            default:
                break;
        }
    }

    static int CompressDir(Vector2 dir) => (int)((int)Math.Ceiling((dir.X + 2) / 2f) * dir.X * dir.X) | (int)((((int)Math.Ceiling((dir.Y + 2) / 2f)) << 2) * dir.Y * dir.Y);
}