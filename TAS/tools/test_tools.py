"""Offline helper tests; no game, Pico, pefile or capstone required."""
import json
from pathlib import Path
import re
import struct
import unittest

from level_points import DEFAULT_INPUT, generate


class LevelPointsTests(unittest.TestCase):
    def test_generated_tables_match_production_except_manual_practice_rows(self):
        data = json.loads(DEFAULT_INPUT.read_text(encoding="utf-8"))
        source = (Path(__file__).resolve().parents[1] / "tas_ui/src/start_line.rs").read_text(encoding="utf-8")
        for kind, name in (("start", "START_POINTS"), ("finish", "FINISH_POINTS"), ("spawn", "SPAWN_CENTROIDS")):
            with self.subTest(kind=kind):
                table = source.split(f"pub const {name}:", 1)[1].split("];", 1)[0]
                expected = self.rows(table)
                expected.pop("PE", None)  # Practice is absent from levelData.json.
                actual = self.rows(generate(data, kind).split("];", 1)[0])
                self.assertEqual(len(actual), 9)
                self.assertEqual(actual, expected)

    @staticmethod
    def rows(table):
        table = re.sub(r"//[^\n]*", "", table)
        return {code: [float(n.replace("_", "")) for n in re.findall(r"-?\d[\d_]*\.\d+", body)]
                for code, body in re.findall(r'\(\s*"([A-Z]+)",\s*(.*?)\)', table, re.S)}

    def test_shared_spawn_clusters_remain_ambiguous(self):
        data = json.loads(DEFAULT_INPUT.read_text(encoding="utf-8"))
        output = generate(data, "spawn")
        for pair in ("AE-AH", "AE-AM", "AH-AM", "FH-FM"):
            self.assertIn(f"# {pair}: 0.0  <-- SHARED", output)

    def test_missing_spawn_is_an_explicit_error(self):
        with self.assertRaisesRegex(ValueError, "ForestEasy has no Player_Start_Location"):
            generate({"ForestEasy": []}, "spawn")


if __name__ == "__main__":
    unittest.main()
