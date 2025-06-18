import { describe, expect, test } from "bun:test";
import { analyzeReplayFile } from "./analyzeReplayFs";
import { readdir, stat } from "fs/promises";
import { relative, join, basename, resolve } from "path";
import { MAX_SCORE } from "./PlaneUtil/scoreTrack";
import { REPLAY_FOLDER } from "./types.node";
import "./generateCircle/toBeCloseToStructure";
// import { sleep } from "bun";
// import { afterAll } from "bun:test";

// await sleep(20*1000);
// console.log("20 more")
// await sleep(20*1000);
// console.log("5 more");
// await sleep(5*1000);
describe("analyzeReplay - tracks", () => {
  const tracks = [
    { name: "AlpineEasy", basePath: resolve(REPLAY_FOLDER, "Alpine/Easy") },
    { name: "AlpineMedium", basePath: resolve(REPLAY_FOLDER, "Alpine/Medium") },
    { name: "AlpineHard", basePath: resolve(REPLAY_FOLDER, "Alpine/Hard") },
    { name: "ForestEasy", basePath: resolve(REPLAY_FOLDER, "Forest/Easy") },
    { name: "Forest MediumOrHard", basePath: resolve(REPLAY_FOLDER, "Forest/Medium") },
    { name: "Forest MediumOrHard", basePath: resolve(REPLAY_FOLDER, "Forest/Hard") },
    { name: "VillageEasy", basePath: resolve(REPLAY_FOLDER, "Village/Easy") },
    { name: "VillageMedium", basePath: resolve(REPLAY_FOLDER, "Village/Medium") },
    { name: "VillageHard", basePath: resolve(REPLAY_FOLDER, "Village/Hard") },
  ];

  tracks.forEach(({ name, basePath }) => {
    describe(name, async () => {
      const dirPath = resolve(process.cwd(), basePath);
      const matchingFiles = await findDatFiles(dirPath);

      if (matchingFiles.length === 0) {
        test.skip(`No files found in ${dirPath}`, () => {});
        return;
      }

      matchingFiles.forEach((filepath) => {
        // if (!filepath.includes("1.15.01")) return;
        test(`identifies "${basename(filepath)}" as ${name}`, async () => {
          const result = await analyzeReplayFile(filepath, {skipCoords: false});
          expect(result.trackName).toBe(name);
          expect(result.trackScoreData?.levelScores[0].score).toBe(MAX_SCORE);
          
          expect(result.trackScoreData!.levelScores[0].scoreData.startPlaneDiffMs).toBe(0);
          expect(result.trackScoreData!.levelScores[0].scoreData.checkpoint1DiffMs).toBe(-10);
          expect(result.trackScoreData!.levelScores[0].scoreData.finishPointDiffMs).toBe(0);
        });
      });
    });
  });

  // afterAll(async () => await sleep(5000*1000));
});


describe("analyzeReplay - other", () => {
  describe("header info", () => {
    test("FM Matt 54.35", async () => {
      const result = await analyzeReplayFile(resolve(REPLAY_FOLDER, "Forest/Medium/0.54.35 Matt.dat"), {skipCoords: true});
      expect(result.playerName).toBe("Matt");
      expect(result.displayedMs).toBe(54350);
      expect(result.startMs).toBe(3400);
      expect(result.totalMs).toBe(57750);

      expect(result.lagBeforeStartMs).toBe(3010);
      expect(result.lagAfterFinishMs).toBe(3020);
      expect(result.recordingMs).toBe(63780);

      expect(result.checkpoint1Ms).toBe(20240);
    });

    test("FM Sesh 1.29.36", async () => {
      const result = await analyzeReplayFile(resolve(REPLAY_FOLDER, "Forest/Medium/1.29.36 SESH.dat"));
      expect(result.playerName).toBe("SESH");
      expect(result.displayedMs).toBe(89360);
      expect(result.startMs).toBe(3410);
      expect(result.totalMs).toBe(92770);

      expect(result.lagBeforeStartMs).toBe(3010);
      expect(result.lagAfterFinishMs).toBe(3030);
      expect(result.recordingMs).toBe(98810);

      expect(result.checkpoint1Ms).toBe(23640);

      expect(result.coords!.rows.length).toEqual(9881);
      expect(result.coords!.rows[0]).toBeCloseToStructure({
        x: 519.2340087890625,
        y: -1555.2247314453125,
        z: 53.577999114990234,
        rotation3x3: [
          [0.9999922513961792, -0.0003098223824054003, 0.003922265954315662],
          [0, 0.9968947172164917, 0.07874537259340286],
          [-0.0039344835095107555, -0.07874476164579391, 0.9968870282173157]
        ]
      });
    });
  });

  // describe.skip("key press", () => {
  //   function expectMatchObject(result: AnalyzeResult, expectedBlock: Partial<RowData>) {
  //     const blocks = result.coords!.rows;
  //     for (let blockIndex = 302; blockIndex <= 602; blockIndex++) {
  //       const block = blocks[blockIndex]!;
  //       expect(block, `BLOCK FAILED\n${block.movementState} ${block.ex}`).toMatchObject(expectedBlock)
  //     }
  //   }

  //   const testCases: Record<string, Partial<RowData>> = {
  //     "KFORRIGHT": {
  //       left: false,
  //       right: true,
  //       forward: true,
  //       shift: false,
  //     },
  //     "KFORLEFT": {
  //       left: true,
  //       right: false,
  //       forward: true,
  //     },
  //     "NOPRESS": {
  //       left: false,
  //       right: false,
  //       forward: false,
  //       shift: false,
  //     },
  //     "KLEFT": {
  //       left: true,
  //       right: false,
  //       forward: false,
  //       shift: false,
  //     },
  //     "KRIGHT": {
  //       left: false,
  //       right: true,
  //       forward: false,
  //       shift: false,
  //     },
  //     "KSHIFTLEFT": {
  //       left: true,
  //       right: false,
  //       forward: false,
  //       shift: true,
  //     },
  //     "KSHIFTRIGHT": {
  //       left: false,
  //       right: true,
  //       forward: false,
  //       shift: true,
  //     }
  //   };
    
  //   Object.entries(testCases).forEach(([testName, expected]) => {
  //     ['vm', 'ae'].forEach(trackname => {
  //       test(`${testName}(${trackname})`, async () => {
  //         expectMatchObject(
  //           await analyzeReplay(join(REPLAY_FOLDER, `tests/keypress/${trackname}/${testName}.dat`), {skipCoords: false}),
  //           expected
  //         );
  //       });
  //     });
  //   });
  // })
});

async function findDatFiles(dirPath: string): Promise<string[]> {
  const files = await readdir(dirPath);
  const results = await Promise.all(
    files.map(async (file) => {
      const fullPath = join(dirPath, file);
      const stats = await stat(fullPath);

      if (stats.isDirectory()) {
        return findDatFiles(fullPath);
      }

      return file.endsWith(".dat") ? [relative(process.cwd(), fullPath).replace(/\\/g, "/")] : [];
    })
  );

  return results.flat();
}
