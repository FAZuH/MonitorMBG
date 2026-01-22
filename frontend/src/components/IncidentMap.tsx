import { useState, useEffect } from 'react';
import { AlertTriangle, Calendar, Users, MapPin, AlertCircle, Info, Map as MapIcon, ZoomIn, ZoomOut, Layers, ExternalLink, Maximize2, X } from 'lucide-react';
import { IncidentMapEducation } from './IncidentMapEducation';
import { ResponsiveMapContainer } from './ResponsiveMapContainer';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './ui/tooltip';
import indonesiaMap from 'figma:asset/d717ab15fae6611f87e941f71178dabee35f18e7.png';

export interface FoodPoisoningIncident {
  id: string;
  location: string;
  province: string;
  date: string;
  victims: number;
  deaths: number;
  cause: string;
  status: 'resolved' | 'investigating' | 'critical';
  coordinates: { x: number; y: number }; // Percentage coordinates (0-100)
}

const incidents: FoodPoisoningIncident[] = [
  {
    id: '1',
    location: 'Jakarta Timur',
    province: 'DKI Jakarta',
    date: '2025-11-15',
    victims: 45,
    deaths: 0,
    cause: 'Kontaminasi bakteri pada makanan sekolah',
    status: 'resolved',
    coordinates: { x: 30.5, y: 66.5 }
  },
  {
    id: '2',
    location: 'Bandung',
    province: 'Jawa Barat',
    date: '2025-11-20',
    victims: 28,
    deaths: 0,
    cause: 'Suhu penyimpanan tidak memadai',
    status: 'resolved',
    coordinates: { x: 27, y: 70 }
  },
  {
    id: '3',
    location: 'Surabaya',
    province: 'Jawa Timur',
    date: '2025-11-22',
    victims: 62,
    deaths: 1,
    cause: 'Kontaminasi silang dalam proses memasak',
    status: 'investigating',
    coordinates: { x: 42, y: 72 }
  },
  {
    id: '4',
    location: 'Semarang',
    province: 'Jawa Tengah',
    date: '2025-11-18',
    victims: 34,
    deaths: 0,
    cause: 'Bahan makanan kadaluarsa',
    status: 'resolved',
    coordinates: { x: 35, y: 67 }
  },
  {
    id: '5',
    location: 'Medan',
    province: 'Sumatera Utara',
    date: '2025-11-10',
    victims: 51,
    deaths: 2,
    cause: 'Keracunan makanan laut',
    status: 'critical',
    coordinates: { x: 7, y: 38 }
  },
  {
    id: '6',
    location: 'Palembang',
    province: 'Sumatera Selatan',
    date: '2025-11-12',
    victims: 19,
    deaths: 0,
    cause: 'Sanitasi peralatan masak tidak memadai',
    status: 'resolved',
    coordinates: { x: 19, y: 64 }
  },
  {
    id: '7',
    location: 'Makassar',
    province: 'Sulawesi Selatan',
    date: '2025-11-25',
    victims: 73,
    deaths: 0,
    cause: 'Kontaminasi air untuk memasak',
    status: 'investigating',
    coordinates: { x: 61, y: 66 }
  },
  {
    id: '8',
    location: 'Denpasar',
    province: 'Bali',
    date: '2025-11-08',
    victims: 15,
    deaths: 0,
    cause: 'Penanganan makanan mentah yang tidak benar',
    status: 'resolved',
    coordinates: { x: 43, y: 75 }
  },
  {
    id: '9',
    location: 'Yogyakarta',
    province: 'DI Yogyakarta',
    date: '2025-11-14',
    victims: 41,
    deaths: 0,
    cause: 'Penyimpanan suhu ruangan terlalu lama',
    status: 'resolved',
    coordinates: { x: 34.5, y: 72 }
  },
  {
    id: '10',
    location: 'Balikpapan',
    province: 'Kalimantan Timur',
    date: '2025-11-26',
    victims: 38,
    deaths: 1,
    cause: 'Kontaminasi pestisida pada sayuran',
    status: 'critical',
    coordinates: { x: 55, y: 35 }
  },
  {
    id: '11',
    location: 'Pontianak',
    province: 'Kalimantan Barat',
    date: '2025-11-05',
    victims: 22,
    deaths: 0,
    cause: 'Kebersihan air tidak memenuhi standar',
    status: 'resolved',
    coordinates: { x: 35.5, y: 42 }
  },
  {
    id: '12',
    location: 'Manado',
    province: 'Sulawesi Utara',
    date: '2025-11-28',
    victims: 31,
    deaths: 0,
    cause: 'Penyimpanan bahan makanan tidak higienis',
    status: 'investigating',
    coordinates: { x: 60, y: 27 }
  },
  {
    id: '13',
    location: 'Banjarmasin',
    province: 'Kalimantan Selatan',
    date: '2025-11-17',
    victims: 27,
    deaths: 0,
    cause: 'Suhu pengolahan tidak mencapai standar',
    status: 'resolved',
    coordinates: { x: 48, y: 55 }
  },
  {
    id: '14',
    location: 'Banda Aceh',
    province: 'Aceh',
    date: '2025-11-03',
    victims: 18,
    deaths: 1,
    cause: 'Kontaminasi pada proses distribusi',
    status: 'critical',
    coordinates: { x: 4, y: 16 }
  },
  {
    id: '15',
    location: 'Pekanbaru',
    province: 'Riau',
    date: '2025-11-21',
    victims: 35,
    deaths: 0,
    cause: 'Peralatan masak tidak disterilisasi dengan baik',
    status: 'investigating',
    coordinates: { x: 12, y: 44 }
  }
];

export function IncidentMap() {
  const [hoveredIncident, setHoveredIncident] = useState<FoodPoisoningIncident | null>(null);
  const [selectedIncident, setSelectedIncident] = useState<FoodPoisoningIncident | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [zoomLevel, setZoomLevel] = useState(1);

  // Prevent body scroll when fullscreen is open
  useEffect(() => {
    if (isFullscreen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }
    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [isFullscreen]);

  const handleZoomIn = () => {
    setZoomLevel(prev => Math.min(prev + 0.2, 3));
  };

  const handleZoomOut = () => {
    setZoomLevel(prev => Math.max(prev - 0.2, 1));
  };

  const handleResetZoom = () => {
    setZoomLevel(1);
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'critical':
        return 'fill-red-500';
      case 'investigating':
        return 'fill-orange-500';
      case 'resolved':
        return 'fill-emerald-500';
      default:
        return 'fill-gray-500';
    }
  };

  const getStatusBadgeColor = (status: string) => {
    switch (status) {
      case 'critical':
        return 'bg-red-100 text-red-800 border-red-300';
      case 'investigating':
        return 'bg-orange-100 text-orange-800 border-orange-300';
      case 'resolved':
        return 'bg-emerald-100 text-emerald-800 border-emerald-300';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-300';
    }
  };

  const totalVictims = incidents.reduce((sum, inc) => sum + inc.victims, 0);
  const totalDeaths = incidents.reduce((sum, inc) => sum + inc.deaths, 0);
  const criticalCases = incidents.filter(inc => inc.status === 'critical').length;
  const investigatingCases = incidents.filter(inc => inc.status === 'investigating').length;

  return (
    <div className="space-y-8">
      {/* Education Section */}
      <IncidentMapEducation />

      {/* Map Section */}
      <div className="bg-white rounded-2xl shadow-lg p-6">
        <div className="mb-6">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 bg-red-100 rounded-lg">
              <AlertCircle className="size-6 text-red-600" />
            </div>
            <div>
              <h2 className="text-gray-900 font-bold">Peta Insiden Keracunan Makanan</h2>
              <p className="text-sm text-gray-600">Data insiden keamanan pangan di Indonesia</p>
            </div>
          </div>
        </div>

        {/* Statistics */}
        <div className="grid grid-cols-4 gap-4 mb-6">
          <div className="bg-gray-50 rounded-xl p-4 border border-gray-200">
            <div className="flex items-center gap-2 mb-1">
              <AlertTriangle className="size-4 text-gray-600" />
              <p className="text-xs text-gray-600 text-[10px]">Total Insiden</p>
            </div>
            <p className="text-2xl text-gray-900">{incidents.length}</p>
          </div>
          <div className="bg-orange-50 rounded-xl p-4 border border-orange-200">
            <div className="flex items-center gap-2 mb-1">
              <Users className="size-4 text-orange-600" />
              <p className="text-xs text-orange-600 text-[10px]">Total Korban</p>
            </div>
            <p className="text-2xl text-orange-900">{totalVictims}</p>
          </div>
          <div className="bg-red-50 rounded-xl p-4 border border-red-200">
            <div className="flex items-center gap-2 mb-1">
              <AlertCircle className="size-4 text-red-600" />
              <p className="text-xs text-red-600 text-[10px]">Kasus Kritis</p>
            </div>
            <p className="text-2xl text-red-900">{criticalCases}</p>
          </div>
          <div className="bg-amber-50 rounded-xl p-4 border border-amber-200">
            <div className="flex items-center gap-2 mb-1">
              <AlertCircle className="size-4 text-amber-600" />
              <p className="text-xs text-amber-600 text-[10px]">Dalam Investigasi</p>
            </div>
            <p className="text-2xl text-amber-900">{investigatingCases}</p>
          </div>
        </div>

        {/* Map Container */}
        <div className="flex gap-6">
          {/* Map */}
          <div className="flex-1 rounded-xl overflow-hidden border-2 border-gray-200 shadow-lg relative bg-white">
            <div className="relative w-full" style={{ paddingBottom: '46%' }}>
              {/* Base Map Image */}
              <img
                src={indonesiaMap}
                alt="Peta Indonesia"
                className="absolute inset-0 w-full h-full object-cover"
              />
              
              {/* Incident Markers Overlay */}
              <div className="absolute inset-0">
                {incidents.map((incident) => (
                  <div
                    key={incident.id}
                    className="absolute cursor-pointer group"
                    style={{
                      left: `${incident.coordinates.x}%`,
                      top: `${incident.coordinates.y}%`,
                      transform: 'translate(-50%, -50%)'
                    }}
                    onMouseEnter={() => setHoveredIncident(incident)}
                    onMouseLeave={() => setHoveredIncident(null)}
                    onClick={() => setSelectedIncident(incident)}
                  >
                    {/* Pulsing outer ring */}
                    <div className={`absolute inset-0 rounded-full ${getStatusColor(incident.status).replace('fill-', 'bg-')} opacity-30 animate-ping`}
                      style={{
                        width: incident.status === 'critical' ? '48px' : '36px',
                        height: incident.status === 'critical' ? '48px' : '36px',
                        left: '50%',
                        top: '50%',
                        transform: 'translate(-50%, -50%)'
                      }}
                    />
                    
                    {/* Main marker */}
                    <div className="relative">
                      <div
                        className={`rounded-full ${getStatusColor(incident.status).replace('fill-', 'bg-')} border-4 border-white shadow-lg group-hover:scale-125 transition-transform`}
                        style={{
                          width: incident.status === 'critical' ? '24px' : '18px',
                          height: incident.status === 'critical' ? '24px' : '18px'
                        }}
                      />
                      
                      {/* Victim count badge */}
                      <div className={`absolute -top-2 -right-2 ${getStatusColor(incident.status).replace('fill-', 'bg-')} text-white text-xs rounded-full px-1.5 py-0.5 min-w-[24px] text-center border-2 border-white shadow-md opacity-0 group-hover:opacity-100 transition-opacity`}>
                        {incident.victims}
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              {/* Hover Tooltip */}
              {hoveredIncident && !selectedIncident && (
                <div className="absolute top-4 right-4 bg-white rounded-lg shadow-xl p-4 border-2 border-emerald-200 max-w-xs z-10 animate-in fade-in slide-in-from-top-2 duration-200">
                  <div className="flex items-start gap-2">
                    <MapPin className="size-5 text-emerald-600 flex-shrink-0 mt-0.5" />
                    <div>
                      <p className="text-gray-900">{hoveredIncident.location}</p>
                      <p className="text-sm text-gray-600">{hoveredIncident.province}</p>
                      <div className="flex items-center gap-2 mt-2">
                        <p className="text-sm text-gray-700">{hoveredIncident.victims} korban</p>
                        {hoveredIncident.deaths > 0 && (
                          <span className="text-xs text-red-600">({hoveredIncident.deaths} meninggal)</span>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {/* Fullscreen Button */}
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <button
                      onClick={() => setIsFullscreen(true)}
                      className="absolute bottom-4 right-4 bg-green-600 hover:bg-green-700 text-white p-3 rounded-lg shadow-lg hover:shadow-xl transition-all duration-200 z-10 group"
                      aria-label="Lihat peta layar penuh"
                    >
                      <Maximize2 className="size-5 transition-transform group-hover:scale-110" />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent 
                    className="hidden md:block"
                    side="left"
                  >
                    <p>Lihat peta layar penuh</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
          </div>

          {/* Legend & Details */}
          <div className="w-80 space-y-4">
            {/* Legend */}
            <div className="bg-gray-50 rounded-xl p-4 border border-gray-200">
              <p className="text-gray-900 mb-3 font-bold">Status Insiden</p>
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 rounded-full bg-red-500"></div>
                  <span className="text-sm text-gray-700">Kritis</span>
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 rounded-full bg-orange-500"></div>
                  <span className="text-sm text-gray-700">Dalam Investigasi</span>
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 rounded-full bg-emerald-500"></div>
                  <span className="text-sm text-gray-700">Terselesaikan</span>
                </div>
              </div>
            </div>

            {/* Selected Incident Details */}
            {selectedIncident && (
              <div className="bg-white rounded-xl p-4 border-2 border-emerald-500 shadow-lg">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-start gap-2">
                    <MapPin className="size-5 text-emerald-600 flex-shrink-0 mt-0.5" />
                    <div>
                      <p className="text-gray-900">{selectedIncident.location}</p>
                      <p className="text-sm text-gray-600">{selectedIncident.province}</p>
                    </div>
                  </div>
                  <button
                    onClick={() => setSelectedIncident(null)}
                    className="text-gray-400 hover:text-gray-600"
                  >
                    ✕
                  </button>
                </div>

                <div className="space-y-3">
                  <div className="flex items-center gap-2 text-sm">
                    <Calendar className="size-4 text-gray-500" />
                    <span className="text-gray-700">{new Date(selectedIncident.date).toLocaleDateString('id-ID', { 
                      year: 'numeric', 
                      month: 'long', 
                      day: 'numeric' 
                    })}</span>
                  </div>

                  <div className="flex items-center gap-2 text-sm">
                    <Users className="size-4 text-gray-500" />
                    <span className="text-gray-700">{selectedIncident.victims} korban</span>
                    {selectedIncident.deaths > 0 && (
                      <span className="text-red-600">({selectedIncident.deaths} meninggal)</span>
                    )}
                  </div>

                  <div className={`inline-block px-3 py-1 rounded-full text-xs border ${getStatusBadgeColor(selectedIncident.status)}`}>
                    {selectedIncident.status === 'critical' && 'Kritis'}
                    {selectedIncident.status === 'investigating' && 'Dalam Investigasi'}
                    {selectedIncident.status === 'resolved' && 'Terselesaikan'}
                  </div>

                  <div className="pt-3 border-t border-gray-200">
                    <p className="text-xs text-gray-600 mb-1">Penyebab:</p>
                    <p className="text-sm text-gray-800">{selectedIncident.cause}</p>
                  </div>
                </div>
              </div>
            )}

            {/* Recent Incidents List */}
            {!selectedIncident && (
              <div className="bg-gray-50 rounded-xl p-4 border border-gray-200 max-h-96 overflow-y-auto">
                <p className="text-gray-900 mb-3">Insiden Terbaru</p>
                <div className="space-y-2">
                  {incidents
                    .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
                    .slice(0, 8)
                    .map((incident) => (
                      <button
                        key={incident.id}
                        onClick={() => setSelectedIncident(incident)}
                        className="w-full text-left p-3 bg-white rounded-lg hover:bg-emerald-50 border border-gray-200 hover:border-emerald-300 transition-colors"
                      >
                        <div className="flex items-start justify-between gap-2">
                          <div className="flex-1 min-w-0">
                            <p className="text-sm text-gray-900 truncate">{incident.location}</p>
                            <p className="text-xs text-gray-600">{incident.province}</p>
                            <p className="text-xs text-gray-600 mt-1">{incident.victims} korban</p>
                          </div>
                          <div className={`w-3 h-3 rounded-full flex-shrink-0 mt-1 ${getStatusColor(incident.status).replace('fill-', 'bg-')}`}></div>
                        </div>
                      </button>
                    ))}
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Full Map Button */}
        <div className="mt-6 flex justify-center">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <button
                  onClick={() => setIsFullscreen(true)}
                  className="
                    inline-flex items-center justify-center gap-3
                    bg-green-600 hover:bg-green-700 active:bg-green-800
                    text-white
                    rounded-xl shadow-lg hover:shadow-xl
                    transition-all duration-200
                    touch-manipulation
                    group
                    px-8 py-3
                    md:px-8 md:py-3
                    w-full md:w-auto
                    min-h-[48px]
                  "
                >
                  <Maximize2 className="size-5 transition-transform group-hover:scale-110" />
                  <span className="font-medium">
                    Lihat Peta Layar Penuh
                  </span>
                </button>
              </TooltipTrigger>
              <TooltipContent 
                className="hidden md:block"
                side="top"
              >
                <p>Buka peta dalam mode layar penuh</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>

        {/* Warning Notice */}
        <div className="mt-6 bg-amber-50 border border-amber-200 rounded-xl p-4">
          <div className="flex gap-3">
            <AlertTriangle className="size-5 text-amber-600 flex-shrink-0 mt-0.5" />
            <div>
              <p className="text-amber-900 font-bold">Perhatian Khusus</p>
              <p className="text-sm text-amber-800 mt-1">
                Data insiden ini dikumpulkan untuk meningkatkan standar keamanan pangan dalam program Makan Bergizi Gratis. 
                Semua dapur wajib mematuhi protokol HACCP untuk mencegah insiden serupa.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Fullscreen Modal */}
      {isFullscreen && (
        <div className="fixed inset-0 bg-black bg-opacity-95 z-50 flex items-center justify-center p-4">
          <div className="relative w-full h-full max-w-[95vw] max-h-[95vh] flex flex-col">
            {/* Header */}
            <div className="flex items-center justify-between mb-4 bg-green-900 bg-opacity-50 backdrop-blur-sm rounded-t-xl p-4">
              <div className="flex items-center gap-3">
                <div className="p-2 bg-red-100 rounded-lg">
                  <AlertCircle className="size-5 text-red-600" />
                </div>
                <div>
                  <h3 className="text-white text-lg">Peta Insiden Keracunan Makanan - Indonesia</h3>
                  <p className="text-sm text-gray-300">Klik marker untuk melihat detail insiden</p>
                </div>
              </div>
              <button
                onClick={() => {
                  setIsFullscreen(false);
                  setZoomLevel(1);
                }}
                className="p-2 hover:bg-white hover:bg-opacity-10 rounded-lg transition-colors"
                aria-label="Tutup peta layar penuh"
              >
                <X className="size-6 text-white" />
              </button>
            </div>

            {/* Map Container */}
            <div className="flex-1 relative bg-white rounded-xl overflow-hidden shadow-2xl">
              {/* Zoom Controls */}
              <div className="absolute top-4 left-4 z-20 flex flex-col gap-2">
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <button
                        onClick={handleZoomIn}
                        disabled={zoomLevel >= 3}
                        className="bg-green-600 hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white p-3 rounded-lg shadow-lg transition-all duration-200"
                        aria-label="Zoom in"
                      >
                        <ZoomIn className="size-5" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent side="right">
                      <p>Perbesar peta</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>

                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <button
                        onClick={handleZoomOut}
                        disabled={zoomLevel <= 1}
                        className="bg-green-600 hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white p-3 rounded-lg shadow-lg transition-all duration-200"
                        aria-label="Zoom out"
                      >
                        <ZoomOut className="size-5" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent side="right">
                      <p>Perkecil peta</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>

                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <button
                        onClick={handleResetZoom}
                        disabled={zoomLevel === 1}
                        className="bg-green-600 hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white p-3 rounded-lg shadow-lg transition-all duration-200"
                        aria-label="Reset zoom"
                      >
                        <Layers className="size-5" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent side="right">
                      <p>Reset zoom</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>

                {/* Zoom level indicator */}
                <div className="bg-white bg-opacity-90 text-gray-700 px-3 py-2 rounded-lg shadow-lg text-sm text-center">
                  {Math.round(zoomLevel * 100)}%
                </div>
              </div>

              <div 
                className="relative w-full h-full overflow-auto"
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center'
                }}
              >
                <div
                  className="relative"
                  style={{
                    width: `${100 * zoomLevel}%`,
                    paddingBottom: `${46 * zoomLevel}%`,
                    minWidth: '100%',
                    minHeight: '100%',
                    transition: 'all 0.3s ease-out'
                  }}
                >
                  {/* Base Map Image */}
                  <img
                    src={indonesiaMap}
                    alt="Peta Indonesia"
                    className="absolute inset-0 w-full h-full object-contain"
                    style={{
                      imageRendering: zoomLevel > 1 ? 'crisp-edges' : 'auto'
                    }}
                  />
                  
                  {/* Incident Markers Overlay */}
                  <div className="absolute inset-0">
                    {incidents.map((incident) => (
                      <div
                        key={incident.id}
                        className="absolute cursor-pointer group"
                        style={{
                          left: `${incident.coordinates.x}%`,
                          top: `${incident.coordinates.y}%`,
                          transform: 'translate(-50%, -50%)'
                        }}
                        onMouseEnter={() => setHoveredIncident(incident)}
                        onMouseLeave={() => setHoveredIncident(null)}
                        onClick={() => setSelectedIncident(incident)}
                      >
                        {/* Pulsing outer ring */}
                        <div 
                          className={`absolute inset-0 rounded-full ${getStatusColor(incident.status).replace('fill-', 'bg-')} opacity-30 animate-ping`}
                          style={{
                            width: incident.status === 'critical' ? '64px' : '48px',
                            height: incident.status === 'critical' ? '64px' : '48px',
                            left: '50%',
                            top: '50%',
                            transform: 'translate(-50%, -50%)'
                          }}
                        />
                        
                        {/* Main marker */}
                        <div className="relative">
                          <div
                            className={`rounded-full ${getStatusColor(incident.status).replace('fill-', 'bg-')} border-4 border-white shadow-lg group-hover:scale-125 transition-transform`}
                            style={{
                              width: incident.status === 'critical' ? '32px' : '24px',
                              height: incident.status === 'critical' ? '32px' : '24px'
                            }}
                          />
                          
                          {/* Victim count badge */}
                          <div className={`absolute -top-3 -right-3 ${getStatusColor(incident.status).replace('fill-', 'bg-')} text-white text-sm rounded-full px-2 py-1 min-w-[32px] text-center border-2 border-white shadow-md opacity-0 group-hover:opacity-100 transition-opacity`}>
                            {incident.victims}
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>

                  {/* Hover Tooltip for Fullscreen */}
                  {hoveredIncident && !selectedIncident && (
                    <div className="absolute top-4 right-4 bg-white rounded-lg shadow-xl p-4 border-2 border-emerald-200 max-w-xs z-10 animate-in fade-in slide-in-from-top-2 duration-200">
                      <div className="flex items-start gap-2">
                        <MapPin className="size-5 text-emerald-600 flex-shrink-0 mt-0.5" />
                        <div>
                          <p className="text-gray-900">{hoveredIncident.location}</p>
                          <p className="text-sm text-gray-600">{hoveredIncident.province}</p>
                          <div className="flex items-center gap-2 mt-2">
                            <p className="text-sm text-gray-700">{hoveredIncident.victims} korban</p>
                            {hoveredIncident.deaths > 0 && (
                              <span className="text-xs text-red-600">({hoveredIncident.deaths} meninggal)</span>
                            )}
                          </div>
                        </div>
                      </div>
                    </div>
                  )}

                  {/* Selected Incident Card in Fullscreen */}
                  {selectedIncident && (
                    <div className="absolute top-4 right-4 bg-white rounded-xl p-5 border-2 border-emerald-500 shadow-2xl max-w-sm z-10 animate-in fade-in slide-in-from-top-2 duration-200">
                      <div className="flex items-start justify-between mb-3">
                        <div className="flex items-start gap-2">
                          <MapPin className="size-5 text-emerald-600 flex-shrink-0 mt-0.5" />
                          <div>
                            <p className="text-gray-900">{selectedIncident.location}</p>
                            <p className="text-sm text-gray-600">{selectedIncident.province}</p>
                          </div>
                        </div>
                        <button
                          onClick={() => setSelectedIncident(null)}
                          className="text-gray-400 hover:text-gray-600 p-1"
                        >
                          <X className="size-5" />
                        </button>
                      </div>

                      <div className="space-y-3">
                        <div className="flex items-center gap-2 text-sm">
                          <Calendar className="size-4 text-gray-500" />
                          <span className="text-gray-700">{new Date(selectedIncident.date).toLocaleDateString('id-ID', { 
                            year: 'numeric', 
                            month: 'long', 
                            day: 'numeric' 
                          })}</span>
                        </div>

                        <div className="flex items-center gap-2 text-sm">
                          <Users className="size-4 text-gray-500" />
                          <span className="text-gray-700">{selectedIncident.victims} korban</span>
                          {selectedIncident.deaths > 0 && (
                            <span className="text-red-600">({selectedIncident.deaths} meninggal)</span>
                          )}
                        </div>

                        <div className={`inline-block px-3 py-1 rounded-full text-xs border ${getStatusBadgeColor(selectedIncident.status)}`}>
                          {selectedIncident.status === 'critical' && 'Kritis'}
                          {selectedIncident.status === 'investigating' && 'Dalam Investigasi'}
                          {selectedIncident.status === 'resolved' && 'Terselesaikan'}
                        </div>

                        <div className="pt-3 border-t border-gray-200">
                          <p className="text-xs text-gray-600 mb-1">Penyebab:</p>
                          <p className="text-sm text-gray-800">{selectedIncident.cause}</p>
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              </div>
            </div>

            {/* Legend */}
            <div className="mt-4 bg-green-900 bg-opacity-50 backdrop-blur-sm rounded-b-xl p-4">
              <div className="flex items-center justify-center gap-8">
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 rounded-full bg-red-500"></div>
                  <span className="text-sm text-white">Kritis ({criticalCases})</span>
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 rounded-full bg-orange-500"></div>
                  <span className="text-sm text-white">Investigasi ({investigatingCases})</span>
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 rounded-full bg-emerald-500"></div>
                  <span className="text-sm text-white">Terselesaikan ({incidents.length - criticalCases - investigatingCases})</span>
                </div>
                <div className="h-6 w-px bg-white bg-opacity-30"></div>
                <div className="flex items-center gap-2">
                  <Users className="size-4 text-white" />
                  <span className="text-sm text-white">Total: {totalVictims} korban</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}