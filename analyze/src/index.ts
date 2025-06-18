import { Command } from "commander";
import { existsSync } from "fs";
import { analyzeReplayFile } from "./analyzeReplayFs";
import { debugKeypress } from "./debugKeypress";
import { dumpObjects } from "./dumpObjects";
import { combine } from "./LevelData/combine";

const program = new Command();

program.name("analyzer").description("CLI tool for file analysis").version("1.0.0");

program
  .command("analyze")
  .description("Analyze a file")
  .argument("<filepath>", "path to file")
  .action(async (filepath: string) => {
    if (!existsSync(filepath)) {
      console.error("File not found:", filepath);
      process.exit(1);
    }

    console.log(await analyzeReplayFile(filepath));
    // writeFileSync("output.txt", JSON.stringify(await analyzeReplay(filepath), null, 2));
    // Add analysis logic here
  });

// program
//   .command("dump")
//   .description("Output file hex")
//   .argument("<filepath>", "path to file")
//   .option("-b, --block <index>", "output specific block by index")
//   .action(async (filepath: string, options: { block: string }) => {
//     if (!existsSync(filepath)) {
//       console.error("File not found:", filepath);
//       process.exit(1);
//     }

//     const content = await readFile(filepath);
//     const hexData = Buffer.from(content.buffer).toString("hex");
//     const blocks = getDataBlocks(hexData);

//     if (options.block) {
//       console.log(blocks[parseInt(options.block)]);
//     } else {
//       console.log(blocks.join("\n")); 
//     }
//     // Add analysis logic here
//   });

program
  .command("debug-keypress")
  .description("Output data for keypress ideas")
  .action(async () => {
    return await debugKeypress();
  })

program
  .command("dump-objects")
  .argument("<filepath>", "path to Object_Data.txt file")
  .argument("[objectNames...]", "Names of object to dump (optional)")
  .option("-f, --format <format>", "Specify a format (original, json)")
  .description("Dump the data from Object_Data.txt in json")
  .action(async (filepath: string, objectNames: Array<string> | undefined, options: {format?: string}) => {
    await dumpObjects(filepath, objectNames, options);
  })

program
  .command("combine")
  .action(async () => {
    await combine();
  })

program.parse();
