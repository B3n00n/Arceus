// React import intentionally omitted as it's not directly referenced
import { cn } from '@/lib/cn';
import { getBatteryColor } from '@/lib/formatting';

interface DeviceBatteryProps {
  level: number; // 0-100
  isCharging?: boolean;
  showLabel?: boolean;
  className?: string;
}

export function DeviceBattery({ level, isCharging = false, showLabel = true, className }: DeviceBatteryProps) {
  const safeLevel = Math.max(0, Math.min(100, Math.round(level)));

  // Visual metrics based on the provided reference
  // Fill container per latest spec
  const innerMaxWidthPx = 13.5; // px
  const innerHeightPx = 6; // px
  const innerLeftPaddingPx = 4; // px
  const innerTopPaddingPx = 9; // px

  const currentFillWidth = Math.max(0, Math.min(innerMaxWidthPx, (safeLevel / 100) * innerMaxWidthPx));
  const fillColorClass = getBatteryColor(safeLevel);

  return (
    <div className={cn('flex items-center gap-2', className)}>
      {/* Battery outline using Material Symbols glyph */}
      <div className="relative w-6 h-6">
        <span className="material-symbols-outlined absolute inset-0 z-0 leading-none text-white/90 text-[24px] flex items-center justify-center select-none">
          battery_android_0
        </span>

        {/* Inner fill area positioned by paddings; only the fill changes color */}
        <div
          className="absolute z-10"
          style={{ top: innerTopPaddingPx, left: innerLeftPaddingPx, height: innerHeightPx, width: innerMaxWidthPx + 'px' }}
        >
          <div
            className={cn('h-full rounded-[1px] transition-[width] duration-300 ease-out', fillColorClass)}
            style={{ width: `${currentFillWidth}px`}}
          />
          {isCharging && (
            <div className="pointer-events-none absolute inset-0 z-20 flex items-center justify-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                width="12"
                height="12"
                aria-hidden="true"
                fill="currentColor"
              >
                <path d="M11 21h-1l1-7H7.5c-.58 0-.57-.32-.38-.66.19-.34.05-.08.07-.12C8.48 10.94 10.42 7.54 13 3h1l-1 7h3.5c.49 0 .56.33.47.51l-.07.15C12.96 17.55 11 21 11 21z"/>
              </svg>
            </div>
          )}
        </div>
      </div>

      {showLabel && (
        <span className="text-xs font-medium text-gray-300">{safeLevel}%</span>
      )}
    </div>
  );
}

export default DeviceBattery;


