import { describe, expect, test } from "bun:test";
import { analyzeReplay } from "./analyzeReplay";
import { readdir, stat } from "fs/promises";
import { relative, join, basename } from "path";

const REPLAY_FOLDER = "../replays";

describe("analyzeReplay", () => {
  const tracks = [
    { name: "Alpine EasyMediumOrHard", basePath: join(REPLAY_FOLDER, "Alpine/Easy") },
    { name: "Alpine EasyMediumOrHard", basePath: join(REPLAY_FOLDER, "Alpine/Medium") },
    { name: "Alpine EasyMediumOrHard", basePath: join(REPLAY_FOLDER, "Alpine/Hard") },
    { name: "Forest Easy", basePath: join(REPLAY_FOLDER, "Forest/Easy") },
    { name: "Forest MediumOrHard", basePath: join(REPLAY_FOLDER, "Forest/Medium") },
    { name: "Forest MediumOrHard", basePath: join(REPLAY_FOLDER, "Forest/Hard") },
    { name: "Village Easy", basePath: join(REPLAY_FOLDER, "Village/Easy") },
    { name: "Village Medium", basePath: join(REPLAY_FOLDER, "Village/Medium") },
    { name: "Village Hard", basePath: join(REPLAY_FOLDER, "Village/Hard") },
  ];

  tracks.forEach(({ name, basePath }) => {
    describe(name, async () => {
      const dirPath = join(process.cwd(), basePath);
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
      const result = await analyzeReplay(join(REPLAY_FOLDER, "Forest/Medium/0.54.35 Matt.dat"));
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
      const result = await analyzeReplay(join(REPLAY_FOLDER, "Forest/Medium/1.29.36 SESH.dat"));
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
