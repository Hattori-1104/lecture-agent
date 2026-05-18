import { Progress } from "@/components/ui/progress"

interface Props {
  value: number
  label?: string
}

export function ProgressBar({ value, label }: Props) {
  const clamped = Math.max(0, Math.min(100, value))
  return (
    <div className="w-full">
      {label && <span className="text-xs text-muted-foreground mb-1 block">{label}</span>}
      <Progress value={clamped} className="h-2" />
      <span className="text-xs text-muted-foreground mt-1 block text-right">{clamped}%</span>
    </div>
  )
}
