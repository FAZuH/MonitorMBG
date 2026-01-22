import { Thermometer, Shuffle, Wrench, UserX, TruckIcon } from 'lucide-react';

type RootCause = 'temperature_storage' | 'cross_contamination' | 'equipment_sanitation' | 'worker_hygiene' | 'supply_chain';

interface RootCauseTagProps {
  causes: RootCause[];
  compact?: boolean;
}

export function RootCauseTag({ causes, compact = false }: RootCauseTagProps) {
  const getRootCauseConfig = (cause: RootCause) => {
    switch (cause) {
      case 'temperature_storage':
        return {
          icon: Thermometer,
          label: 'Suhu Penyimpanan',
          color: 'text-red-700',
          bg: 'bg-red-100',
          border: 'border-red-300'
        };
      case 'cross_contamination':
        return {
          icon: Shuffle,
          label: 'Kontaminasi Silang',
          color: 'text-orange-700',
          bg: 'bg-orange-100',
          border: 'border-orange-300'
        };
      case 'equipment_sanitation':
        return {
          icon: Wrench,
          label: 'Peralatan & Sanitasi',
          color: 'text-blue-700',
          bg: 'bg-blue-100',
          border: 'border-blue-300'
        };
      case 'worker_hygiene':
        return {
          icon: UserX,
          label: 'Higiene Pekerja',
          color: 'text-purple-700',
          bg: 'bg-purple-100',
          border: 'border-purple-300'
        };
      case 'supply_chain':
        return {
          icon: TruckIcon,
          label: 'Supply Chain',
          color: 'text-teal-700',
          bg: 'bg-teal-100',
          border: 'border-teal-300'
        };
    }
  };

  if (!causes || causes.length === 0) {
    return (
      <div className="px-3 py-2 bg-gray-100 border border-gray-300 rounded-lg">
        <p className="text-xs text-gray-600">Tidak ada penyebab yang ditandai</p>
      </div>
    );
  }

  if (compact) {
    return (
      <div className="flex flex-wrap gap-2">
        {causes.map((cause, idx) => {
          const config = getRootCauseConfig(cause);
          const Icon = config.icon;
          return (
            <div
              key={idx}
              className={`flex items-center gap-1 px-2 py-1 rounded-lg border ${config.bg} ${config.border}`}
            >
              <Icon className={`size-3 ${config.color}`} />
              <span className={`text-xs ${config.color}`}>{config.label}</span>
            </div>
          );
        })}
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <h4 className="text-gray-900 flex items-center gap-2">
        <span className="w-1 h-5 bg-emerald-600 rounded-full"></span>
        Root Cause Analysis
      </h4>
      <div className="space-y-2">
        {causes.map((cause, idx) => {
          const config = getRootCauseConfig(cause);
          const Icon = config.icon;
          return (
            <div
              key={idx}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl border ${config.bg} ${config.border}`}
            >
              <div className={`p-2 ${config.bg} rounded-lg`}>
                <Icon className={`size-5 ${config.color}`} />
              </div>
              <div className="flex-1">
                <p className={`text-sm ${config.color}`}>{config.label}</p>
                <p className="text-xs text-gray-600">Kategori HACCP</p>
              </div>
            </div>
          );
        })}
      </div>
      <div className="bg-blue-50 border border-blue-200 rounded-xl p-3">
        <p className="text-xs text-blue-800">
          💡 <strong>Catatan:</strong> Penyebab ini ditandai berdasarkan analisis HACCP untuk mencegah tuduhan umum tanpa bukti.
        </p>
      </div>
    </div>
  );
}
