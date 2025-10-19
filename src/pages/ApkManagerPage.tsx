import { Package } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';

export function ApkManagerPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-white">APK Manager</h1>
        <p className="text-gray-400 mt-1">Manage and deploy APK files</p>
      </div>

      <Card className="bg-discord-dark-2 border-discord-dark">
        <CardContent className="p-12 text-center">
          <Package className="h-12 w-12 mx-auto text-gray-600 mb-4" />
          <h3 className="text-lg font-semibold text-white mb-2">
            APK Manager - Coming Soon
          </h3>
          <p className="text-gray-400">
            This feature is under development
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
