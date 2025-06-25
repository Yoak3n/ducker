import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader
} from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
export default function ActionCard() {
    return (
    <Card >
      <CardHeader>
        <CardAction>
          <Button variant="link">Delete</Button>
        </CardAction>
      </CardHeader>
      <CardContent>
      </CardContent>
      <CardFooter className="flex-col gap-2">
        <Button type="submit" className="w-1/2">
          Submit
        </Button>
      </CardFooter>
    </Card>
    )
}