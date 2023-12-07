class Day5
{
    public static void Start(string[] args)
    {
        string txtPath = @"input5.txt";

        List<string> sections = File.ReadAllText(txtPath).Split("\n\r\n").ToList();
        List<uint> seedNums = sections[0].Split(": ")[1].Split(" ").Select(uint.Parse).ToList();
        sections.RemoveAt(0);

        List<uint> lowestLoc = new();

        //int start = int.Parse(args[0]) * 2;
        //for (int i = start - 1, index = 0; i < start; i += 2, index++)
        for (int i = 1, index = 0; i < seedNums.Count; i += 2, index++)
        {
            uint numSeeds = seedNums[i];

            uint[] sourceArr = new uint[numSeeds],
                    destArr = new uint[numSeeds];

            uint startIndex = seedNums[i - 1];
            uint seedCount = seedNums[i];

            for (int j = 0; j < seedCount; j++)
                sourceArr[j] = (uint)(startIndex + j);

            for (int j = 0; j < 7; j++)
            {
                string[] lines = sections[j].Split("\n")[1..];
                for (int k = 0; k < lines.Length; k++)
                {
                    uint[] nums = lines[k].Split(" ").Select(uint.Parse).ToArray();

                    for (int l = 0; l < numSeeds; l++)
                    {
                        if (sourceArr[l] >= nums[1])
                        {
                            uint difference = sourceArr[l] - nums[1];
                            if (difference < nums[2])
                                destArr[l] = nums[0] + difference;
                        }
                    }
                }

                for (int k = 0; k < numSeeds; k++)
                    if (destArr[k] == default)
                        destArr[k] = sourceArr[k];

                destArr.CopyTo(sourceArr, 0);
            }
            lowestLoc.Add(uint.MaxValue);
            destArr.ToList().ForEach(x => lowestLoc[index] = x < lowestLoc[index] ? x : lowestLoc[index]);
        }
        uint lowest = uint.MaxValue;

        lowestLoc.ToList().ForEach(x => lowest = x < lowest ? x : lowest);

        Console.Write(lowest);
    }
}