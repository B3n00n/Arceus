import { Package } from 'lucide-react';

export function ApkManagerPage() {
  return (
    <div className="space-y-6 p-6">
      <div>
        <p className="text-grey-300 mt-1">Manage and deploy APK files</p>
      </div>

      <div className="rounded-lg border bg-grey-800 border-grey-600 shadow p-12 text-center">
        <Package className="h-12 w-12 mx-auto text-grey-400 mb-4" />
        <h3 className="text-lg font-semibold text-white mb-2">
          APK Manager - Coming Soon
        </h3>
        <p className="text-grey-300">
          This feature is under development
        </p>
      </div>
    </div>
  );
}
