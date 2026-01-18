import { Settings as SettingsIcon } from 'lucide-react';

export function SettingsPage() {
  return (
    <div className="space-y-6 p-6">
      <div>
        <p className="text-grey-300 mt-1">Configure application preferences</p>
      </div>

      <div className="rounded-lg border bg-grey-800 border-grey-600 shadow p-12 text-center">
        <SettingsIcon className="h-12 w-12 mx-auto text-grey-400 mb-4" />
        <h3 className="text-lg font-semibold text-white mb-2">
          Settings - Coming Soon
        </h3>
        <p className="text-grey-300">
          This feature is under development
        </p>
      </div>
    </div>
  );
}
