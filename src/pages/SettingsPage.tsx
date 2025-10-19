import { Settings as SettingsIcon } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';

export function SettingsPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-white">Settings</h1>
        <p className="text-gray-400 mt-1">Configure application preferences</p>
      </div>

      <Card className="bg-discord-dark-2 border-discord-dark">
        <CardContent className="p-12 text-center">
          <SettingsIcon className="h-12 w-12 mx-auto text-gray-600 mb-4" />
          <h3 className="text-lg font-semibold text-white mb-2">
            Settings - Coming Soon
          </h3>
          <p className="text-gray-400">
            This feature is under development
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
