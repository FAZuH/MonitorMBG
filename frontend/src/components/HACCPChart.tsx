import { HACCPRating } from '../App';

interface HACCPChartProps {
  ratings: HACCPRating;
}

export function HACCPChart({ ratings }: HACCPChartProps) {
  const criteria = [
    { key: 'taste', label: 'Taste', labelId: 'Cita Rasa', value: ratings.taste, color: 'from-rose-500 to-pink-500' },
    { key: 'hygiene', label: 'Hygiene', labelId: 'Kebersihan', value: ratings.hygiene, color: 'from-blue-500 to-cyan-500' },
    { key: 'freshness', label: 'Freshness', labelId: 'Kesegaran', value: ratings.freshness, color: 'from-green-500 to-emerald-500' },
    { key: 'temperature', label: 'Temperature', labelId: 'Suhu', value: ratings.temperature, color: 'from-orange-500 to-amber-500' },
    { key: 'packaging', label: 'Packaging', labelId: 'Kemasan', value: ratings.packaging, color: 'from-purple-500 to-violet-500' },
    { key: 'handling', label: 'Handling', labelId: 'Penanganan', value: ratings.handling, color: 'from-teal-500 to-cyan-500' }
  ];

  const maxValue = 5;

  return (
    <div className="space-y-6">
      {criteria.map((criterion) => (
        <div key={criterion.key}>
          <div className="flex items-center justify-between mb-2">
            <div>
              <p className="text-emerald-900">{criterion.label}</p>
              <p className="text-sm text-emerald-600">{criterion.labelId}</p>
            </div>
            <div className="text-right">
              <p className="text-emerald-900">{criterion.value.toFixed(1)}</p>
              <p className="text-xs text-emerald-600">/ {maxValue}</p>
            </div>
          </div>
          <div className="h-4 bg-emerald-100 rounded-full overflow-hidden">
            <div
              className={`h-full bg-gradient-to-r ${criterion.color} rounded-full transition-all duration-700 ease-out`}
              style={{ width: `${(criterion.value / maxValue) * 100}%` }}
            />
          </div>
        </div>
      ))}

      <div className="pt-6 border-t border-emerald-200 mt-8">
        <div className="flex items-center justify-between">
          <p className="text-emerald-900">Overall HACCP Score</p>
          <p className="text-emerald-900">
            {(Object.values(ratings).reduce((a, b) => a + b, 0) / 6).toFixed(2)} / 5.00
          </p>
        </div>
        <div className="h-6 bg-emerald-100 rounded-full overflow-hidden mt-3">
          <div
            className="h-full bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 rounded-full transition-all duration-700 ease-out"
            style={{ 
              width: `${(Object.values(ratings).reduce((a, b) => a + b, 0) / 6 / maxValue) * 100}%` 
            }}
          />
        </div>
      </div>
    </div>
  );
}
