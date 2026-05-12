#!/usr/bin/env python3
"""
Stop hook: 数学の会話が含まれていた場合に /save-progress の実行を促す。
Claude API は使用しない。
"""

import json
import sys
from pathlib import Path

MATH_KEYWORDS = [
    "定理", "証明", "行列", "固有値", "積分", "微分", "極限", "確率", "群", "環", "位相",
    "ベクトル", "線形", "関数", "解析", "方程式", "写像", "集合", "収束", "展開",
    "フーリエ", "ラプラス", "テイラー", "コーシー", "jordan", "Jordan",
]


def main() -> None:
    try:
        hook_input = json.loads(sys.stdin.read())
    except (json.JSONDecodeError, OSError):
        sys.exit(0)

    transcript_path = hook_input.get("transcript_path", "")
    if not transcript_path or not Path(transcript_path).exists():
        sys.exit(0)

    try:
        content = Path(transcript_path).read_text(encoding="utf-8")
    except OSError:
        sys.exit(0)

    if any(kw in content for kw in MATH_KEYWORDS):
        print(json.dumps({
            "systemMessage": "数学の学習内容が検出されました。/save-progress を実行すると進捗が記録されます。"
        }))


if __name__ == "__main__":
    main()
