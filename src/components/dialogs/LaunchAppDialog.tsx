import { useState } from "react"
import { Checkbox } from "@/components/ui/checkbox"
import { DialogOverlay } from "./DialogOverlay"
import { DialogWindow, DialogHeader, DialogContent, DialogFooter } from "./DialogWindow"
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
      <DialogWindow className="w-120">
        <DialogHeader title="Launch App" />
        <DialogContent className="space-y-5">
          {/* Dropdown */}
          <div>
            <label className="text-sm text-grey-200 mb-2 block">
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
            <span className="text-sm text-grey-200">
              Also launch on connected devices
            </span>
          </div>
        </DialogContent>
        <DialogFooter
          confirmText="Launch"
          onConfirm={handleLaunch}
          confirmDisabled={loading || !selectedApp}
          onCancel={onClose}
          cancelDisabled={loading}
        />
      </DialogWindow>
    </DialogOverlay>
  )
}
