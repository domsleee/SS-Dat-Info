import { describe, expect, test } from "bun:test";
import { analyzeReplay } from "./analyzeReplayFs";
import { readdir, stat } from "fs/promises";
import { relative, join, basename, resolve } from "path";
import { AnalyzeResult, REPLAY_FOLDER, RowData } from "./types";


describe("analyzeReplay", () => {
  const tracks = [
    { name: "Alpine EasyMediumOrHard", basePath: resolve(REPLAY_FOLDER, "Alpine/Easy") },
    { name: "Alpine EasyMediumOrHard", basePath: resolve(REPLAY_FOLDER, "Alpine/Medium") },
    { name: "Alpine EasyMediumOrHard", basePath: resolve(REPLAY_FOLDER, "Alpine/Hard") },
    { name: "Forest Easy", basePath: resolve(REPLAY_FOLDER, "Forest/Easy") },
    { name: "Forest MediumOrHard", basePath: resolve(REPLAY_FOLDER, "Forest/Medium") },
    { name: "Forest MediumOrHard", basePath: resolve(REPLAY_FOLDER, "Forest/Hard") },
    { name: "Village Easy", basePath: resolve(REPLAY_FOLDER, "Village/Easy") },
    { name: "Village Medium", basePath: resolve(REPLAY_FOLDER, "Village/Medium") },
    { name: "Village Hard", basePath: resolve(REPLAY_FOLDER, "Village/Hard") },
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
        test(`identifies "${basename(filepath)}" as ${name}`, async () => {
          const result = await analyzeReplay(filepath);
          expect(result.trackName).toBe(name);
        });
      });
    });
  });

  describe("header info", () => {
    test("FM Matt 54.35", async () => {
      const result = await analyzeReplay(resolve(REPLAY_FOLDER, "Forest/Medium/0.54.35 Matt.dat"));
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
      const result = await analyzeReplay(resolve(REPLAY_FOLDER, "Forest/Medium/1.29.36 SESH.dat"));
      expect(result.playerName).toBe("SESH");
      expect(result.displayedMs).toBe(89360);
      expect(result.startMs).toBe(3410);
      expect(result.totalMs).toBe(92770);

      expect(result.lagBeforeStartMs).toBe(3010);
      expect(result.lagAfterFinishMs).toBe(3030);
      expect(result.recordingMs).toBe(98810);

      expect(result.checkpoint1Ms).toBe(23640);
    });
  });

  describe.skip("key press", () => {
    function expectMatchObject(result: AnalyzeResult, expectedBlock: Partial<RowData>) {
      const blocks = result.coords!.rows;
      for (let blockIndex = 302; blockIndex <= 602; blockIndex++) {
        const block = blocks[blockIndex]!;
        expect(block, `BLOCK FAILED\n${block.movementState} ${block.ex}`).toMatchObject(expectedBlock)
      }
    }

    const testCases: Record<string, Partial<RowData>> = {
      "KFORRIGHT": {
        left: false,
        right: true,
        forward: true,
        shift: false,
      },
      "KFORLEFT": {
        left: true,
        right: false,
        forward: true,
      },
      "NOPRESS": {
        left: false,
        right: false,
        forward: false,
        shift: false,
      },
      "KLEFT": {
        left: true,
        right: false,
        forward: false,
        shift: false,
      },
      "KRIGHT": {
        left: false,
        right: true,
        forward: false,
        shift: false,
      },
      "KSHIFTLEFT": {
        left: true,
        right: false,
        forward: false,
        shift: true,
      },
      "KSHIFTRIGHT": {
        left: false,
        right: true,
        forward: false,
        shift: true,
      }
    };
    
    Object.entries(testCases).forEach(([testName, expected]) => {
      ['vm', 'ae'].forEach(trackname => {
        test(`${testName}(${trackname})`, async () => {
          expectMatchObject(
            await analyzeReplay(join(REPLAY_FOLDER, `tests/keypress/${trackname}/${testName}.dat`), true),
            expected
          );
        });
      });
    });
  })
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
