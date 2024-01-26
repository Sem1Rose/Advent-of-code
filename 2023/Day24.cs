using System.Windows;

class Day24
{
    static string txtPath = @"input24.txt";
    static (decimal x, decimal y)[] hailstonePos;
    static (decimal x, decimal y)[] hailstoneVel;

    static void Main()
    {
        string[] lines = File.ReadAllLines(txtPath);

        hailstonePos = lines.Select(x =>
                                        (
                                            decimal.Parse(x.Split(", ")[0]) * 1e-14m,
                                            decimal.Parse(x.Split(", ")[1]) * 1e-14m
                                        )
                                    ).ToArray();

        hailstoneVel = lines.Select(x =>
                                        (
                                            (decimal.Parse(x.Split(" @ ")[1].Split(", ")[0]) + decimal.Parse(x.Split(", ")[0])) * 1e-14m,
                                            (decimal.Parse(x.Split(" @ ")[1].Split(", ")[1]) + decimal.Parse(x.Split(", ")[1])) * 1e-14m
                                        )
                                    ).ToArray();

        Console.WriteLine(PartOne());
        Console.Read();
    }

    // 6697
    // 6700
    // 6698
    // 13149
    static int PartOne()
    {
        int numIntersections = 0;
        for (int i = 0; i < hailstonePos.Length - 1; i++)
        {
            for (int j = i + 1; j < hailstonePos.Length; j++)
            {
                decimal determinant = (hailstonePos[i].x - hailstoneVel[i].x) * (hailstonePos[j].y - hailstoneVel[j].y) - (hailstonePos[i].y - hailstoneVel[i].y) * (hailstonePos[j].x - hailstoneVel[j].x);

                if (determinant != 0)
                {
                    // Thanks, Wikipedia: https://en.wikipedia.org/wiki/Line–line_intersection#Given_two_points_on_each_line_segment
                    (decimal x, decimal y) intersectionPoint = (
                        ((hailstonePos[i].x * hailstoneVel[i].y - hailstonePos[i].y * hailstoneVel[i].x) * (hailstonePos[j].x - hailstoneVel[j].x) - (hailstonePos[i].x - hailstoneVel[i].x) * (hailstonePos[j].x * hailstoneVel[j].y - hailstonePos[j].y * hailstoneVel[j].x)) / determinant,
                        ((hailstonePos[i].x * hailstoneVel[i].y - hailstonePos[i].y * hailstoneVel[i].x) * (hailstonePos[j].y - hailstoneVel[j].y) - (hailstonePos[i].y - hailstoneVel[i].y) * (hailstonePos[j].x * hailstoneVel[j].y - hailstonePos[j].y * hailstoneVel[j].x)) / determinant
                    );

                    if (intersectionPoint.x >= 2e0m && intersectionPoint.y >= 2e0m && intersectionPoint.x <= 4e0m && intersectionPoint.y <= 4e0m)
                    {
                        (decimal x, decimal y) dira = (intersectionPoint.x - hailstonePos[i].x, intersectionPoint.y - hailstonePos[i].y);
                        (decimal x, decimal y) origa = (hailstoneVel[i].x - hailstonePos[i].x, hailstoneVel[i].y - hailstonePos[i].y);
                        (decimal x, decimal y) dirb = (intersectionPoint.x - hailstonePos[j].x, intersectionPoint.y - hailstonePos[j].y);
                        (decimal x, decimal y) origb = (hailstoneVel[j].x - hailstonePos[j].x, hailstoneVel[j].y - hailstonePos[j].y);

                        decimal lineAdot = dira.x * origa.x + dira.y * origa.y;
                        decimal lineBdot = dirb.x * origb.x + dirb.y * origb.y;

                        if (lineAdot > 0 && lineBdot > 0)
                            numIntersections++;
                    }
                }
            }
        }
        return numIntersections;
    }
}