import { Badge } from "@/components/ui/badge"

interface Session {
  id: string
  date: string
  topicsCovered: string[]
  problemsAttempted: number
  problemsCorrect: number
  summary: string
  weakPointsIdentified: string[]
}

interface Props {
  sessions: Session[]
}

export function SessionHistory({ sessions }: Props) {
  const recent = [...sessions]
    .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
    .slice(0, 10)

  if (recent.length === 0) {
    return <p className="text-muted-foreground text-sm">まだセッションがありません。</p>
  }

  return (
    <div className="flex flex-col">
      {recent.map((s, index) => {
        const d = new Date(s.date)
        const dateStr = d.toLocaleDateString('ja-JP', { year: 'numeric', month: 'short', day: 'numeric' })
        const timeStr = d.toLocaleTimeString('ja-JP', { hour: '2-digit', minute: '2-digit' })
        const accuracy = s.problemsAttempted > 0
          ? Math.round((s.problemsCorrect / s.problemsAttempted) * 100)
          : null
        const isLast = index === recent.length - 1

        return (
          <div key={s.id} className="flex gap-4">
            {/* ドット＋線のカラム */}
            <div className="flex flex-col items-center w-4 shrink-0">
              <div className="w-2.5 h-2.5 rounded-full bg-primary shrink-0 mt-1.5" />
              {!isLast && <div className="w-0.5 bg-border grow mt-1.5" />}
            </div>
            {/* コンテンツ */}
            <div className={isLast ? "pb-0 flex-1" : "pb-6 flex-1"}>
              <div className="text-xs text-muted-foreground mb-1">{dateStr} {timeStr}</div>
              <p className="text-sm text-foreground mb-2">{s.summary}</p>
              <div className="flex flex-wrap gap-1.5">
                {s.topicsCovered.map((t) => (
                  <Badge key={t} variant="outline">{t}</Badge>
                ))}
                {accuracy !== null && (
                  <Badge variant="secondary">正解 {accuracy}% ({s.problemsCorrect}/{s.problemsAttempted})</Badge>
                )}
              </div>
            </div>
          </div>
        )
      })}
    </div>
  )
}
