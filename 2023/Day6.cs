class Day6
{
    public static void Start()
    {
        string txtPath = @"input6.txt";
        string[] lines = File.ReadAllLines(txtPath);

        string timesStr = string.Join("", lines[0].Split(" ")[1..]);
        string distancesStr = string.Join("", lines[1].Split(" ")[1..]);

        long waysToWin = 0;
        long time = long.Parse(timesStr);
        long distance = long.Parse(distancesStr);

        long startingT = (int)Math.Ceiling((time - Math.Sqrt(time * time - 4 * distance)) / 2);

        for (long t = startingT; t < time; t++)
        {
            long d = t * time - t * t;
            if (d <= distance)
                if (waysToWin != 0)
                    break;
                else
                    continue;
            waysToWin++;
        }

        Console.WriteLine(waysToWin);
    }
}