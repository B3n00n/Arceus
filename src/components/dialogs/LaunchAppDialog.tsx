import { useState } from "react"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { DialogOverlay } from "./DialogOverlay"
import { Dropdown } from "@/components/ui/dropdown"

export interface LaunchAppDialogProps {
  isOpen: boolean
  onClose: () => void
  availableApps: { name: string; packageName: string }[]
  onLaunch: (app: { name: string; packageName: string }, launchOnClients: boolean) => void
  loading?: boolean
}

export function LaunchAppDialog({
  isOpen,
  onClose,
  availableApps,
  onLaunch,
  loading = false,
}: LaunchAppDialogProps) {
  const [selectedApp, setSelectedApp] = useState<string>("")
  const [launchOnClients, setLaunchOnClients] = useState(false)

  if (!isOpen) return null

  const handleLaunch = () => {
    const app = availableApps.find((a) => a.name === selectedApp)
    if (!app) return
    onLaunch(app, launchOnClients)
  }

  return (
    <DialogOverlay onClose={onClose}>
      <Card className="w-120">
        <CardHeader className="pb-3">
          <h3 className="text-lg font-semibold text-white">Launch App</h3>
        </CardHeader>

        <CardContent className="space-y-5">
          {/* Dropdown */}
          <div>
            <label className="text-sm text-gray-300 mb-2 block">
              Choose an app to launch on the server
            </label>
            <Dropdown
              options={availableApps.map((a) => a.name)}
              value={selectedApp}
              onChange={setSelectedApp}
              placeholder="Choose an app"
              disabled={loading}
            />
          </div>

          {/* Checkbox */}
          <div className="flex items-center pb-2 space-x-2">
            <Checkbox
              checked={launchOnClients}
              onCheckedChange={() => setLaunchOnClients(!launchOnClients)}
              disabled={loading}
            />
            <span className="text-sm text-gray-300">
              Also launch on connected devices
            </span>
          </div>

          {/* Actions */}
          <div className="flex flex-row-reverse justify-between gap-2 pt-1">
            <Button
              variant="default"
              onClick={handleLaunch}
              disabled={loading || !selectedApp}
            >             
              Launch
            </Button>
            <Button
              variant="outline"
              onClick={onClose}
              disabled={loading}
            >
              Cancel
            </Button>
          </div>
        </CardContent>
      </Card>
    </DialogOverlay>
  )
}
