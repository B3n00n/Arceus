import { GamesSection } from '../components/games/GamesSection';

export function DashboardPage() {
  return (
    <div className="relative min-h-[calc(100vh-4rem)] p-6">
      <div className="mb-8">
        <h2 className="text-xl font-bold text-white">My Apps</h2>
      </div>

      {/* Games Section */}
      <GamesSection />
    </div>
  );
}
