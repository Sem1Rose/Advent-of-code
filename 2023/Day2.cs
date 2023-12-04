class Day2
{
    readonly static string[] colors = new string[3] { "red", "green", "blue" };
    readonly static int[] inventory = new int[3] { 12, 13, 14 };

    public static void Start()
    {
        string txtPath = @"input2.txt";
        string[] games = File.ReadAllLines(txtPath);
        int sum = 0;

        for (int i = 0; i < games.Length; i++)
        {
            string[] sets = games[i].Split(": ")[1].Split("; ");
            int[] minimumInventory = new int[3] { 0, 0, 0 };

            foreach (var set in sets)
            {
                string[] cubes = set.Split(", ");

                foreach (var cube in cubes)
                {
                    string[] split = cube.Split(" ");

                    for (int j = 0; j < colors.Length; j++)
                    {
                        if (split[1].SequenceEqual(colors[j]))
                        {
                            int count = int.Parse(split[0]);
                            if (count > minimumInventory[j])
                                minimumInventory[j] = count;

                            break;
                        }
                    }
                }
            }

            sum += minimumInventory[0] * minimumInventory[1] * minimumInventory[2];
        }

        Console.WriteLine(sum);
        Console.Read();
    }
}