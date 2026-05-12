---
description: このセッションの学習内容を分析してJSONに記録し、GitHubにpushする
allowed-tools: Write, Edit, Bash, Read
---

このセッションの会話全体を振り返り、以下の手順で学習記録を保存してください。

## ステップ 1: セッションを分析する

会話を確認し、以下を特定してください:

- **数学セッションかどうか**: 数学の学習が主な内容でない場合（コーディングや雑談のみ）は「数学のセッションではないため記録をスキップします」と伝えて終了してください
- **扱ったトピック**: `分野/小トピック` 形式で列挙（例: `線形代数/固有値`、`微積分/多変数積分`）
- **問題数**: 試行数と正解数（問題を解いていない場合は 0）
- **トピックごとの理解度**: 0〜100 で推定（正解率・解答の質・詰まり具合を総合的に判断）
- **苦手点**: 具体的に詰まっていた概念や計算ステップ
- **要約**: 日本語で 2〜3 文

## ステップ 2: セッションファイルを作成する

現在時刻を `YYYY-MM-DDTHHMMSS` 形式にして `data/sessions/` 以下に保存してください。

ファイル内容:
```json
{
  "id": "<セッションの識別子（タイムスタンプで可）>",
  "date": "<ISO 8601 形式の現在時刻>",
  "topicsCovered": ["<トピック1>", "<トピック2>"],
  "problemsAttempted": 0,
  "problemsCorrect": 0,
  "summary": "<要約>",
  "weakPointsIdentified": ["<苦手点1>", "<苦手点2>"]
}
```

## ステップ 3: progress.json を更新する

`data/progress.json` を Read で読み込み、以下のルールで更新してください:

- **topics**: 各トピックを更新・追加する
  - 初回: `{ understanding, attempts: 0, correct: 0, weakPoints: [], lastPracticed }`
  - 2回目以降の理解度: `new = round(prev * 0.6 + current * 0.4)` （直近セッション重み付け平均）
  - `weakPoints`: 既存リストと新しいリストを union（重複除去）
  - `attempts` / `correct` は累積加算
  - `lastPracticed`: 今日の日付
- **overallStats**: `totalSessions +1`、`totalProblems` と `totalCorrect` を加算
- **lastUpdated**: 現在の ISO 8601 時刻

## ステップ 4: git add → commit → push

```bash
git -C /home/hattori/repositories/lecture-agent add data/
git -C /home/hattori/repositories/lecture-agent commit -m "学習記録: <YYYY-MM-DD HH:MM>"
git -C /home/hattori/repositories/lecture-agent push
```

push が成功したら「記録を保存しました」と結果を要約して報告してください。
push に失敗した場合（リモート未設定など）は失敗した旨を伝え、ファイルへの書き込みは完了済みであることを伝えてください。
