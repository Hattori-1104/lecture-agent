import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ProgressBar } from "./ProgressBar"

interface Props {
  name: string
  understanding: number
  attempts: number
  correct: number
  weakPoints: string[]
  lastPracticed: string | null
}

export function TopicCard({ name, understanding, attempts, correct, weakPoints, lastPracticed }: Props) {
  const accuracy = attempts > 0 ? Math.round((correct / attempts) * 100) : null

  return (
    <Card size="sm">
      <CardHeader>
        <div className="flex items-start justify-between gap-2">
          <CardTitle>{name}</CardTitle>
          {lastPracticed && (
            <span className="text-xs text-muted-foreground whitespace-nowrap shrink-0 pt-0.5">{lastPracticed}</span>
          )}
        </div>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <ProgressBar value={understanding} label="理解度" />
        {accuracy !== null && (
          <div className="text-sm text-muted-foreground">
            正解率 <span className="text-foreground font-medium">{accuracy}%</span>
            <span className="mx-1">·</span>
            {correct}/{attempts} 問
          </div>
        )}
        {weakPoints.length > 0 && (
          <div>
            <p className="text-xs text-muted-foreground mb-1.5">苦手点</p>
            <ul className="flex flex-wrap gap-1.5">
              {weakPoints.map((wp) => (
                <li key={wp}>
                  <Badge variant="destructive">{wp}</Badge>
                </li>
              ))}
            </ul>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
