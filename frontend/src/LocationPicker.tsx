import { useState } from 'react';
import { MapContainer, TileLayer, Marker, useMapEvents } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import L, { LatLng } from 'leaflet';

// Fix for default marker icon in Leaflet with React
import icon from 'leaflet/dist/images/marker-icon.png';
import iconShadow from 'leaflet/dist/images/marker-shadow.png';

let DefaultIcon = L.icon({
    iconUrl: icon,
    shadowUrl: iconShadow,
    iconSize: [25, 41],
    iconAnchor: [12, 41]
});
L.Marker.prototype.options.icon = DefaultIcon;

interface LocationPickerProps {
  onLocationSelect: (latlng: string) => void;
  onClose: () => void;
}

function MapEvents({ setPosition }: { setPosition: (pos: LatLng) => void }) {
  useMapEvents({
    click(e: L.LeafletMouseEvent) {
      setPosition(e.latlng);
    },
  });
  return null;
}

export default function LocationPicker({ onLocationSelect, onClose }: LocationPickerProps) {
  const [position, setPosition] = useState<LatLng | null>(null);

  const handleConfirm = () => {
    if (position) {
      onLocationSelect(`${position.lat},${position.lng}`);
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div className="bg-white w-full max-w-lg p-6 rounded-3xl shadow-2xl">
        <h3 className="text-xl font-bold mb-4">Select Location</h3>
        <div className="h-64 mb-4 rounded-xl overflow-hidden">
          <MapContainer center={[12.9716, 77.5946] as L.LatLngExpression} zoom={13} style={{ height: '100%', width: '100%' }}>
            <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
            {position && <Marker position={position} />}
            <MapEvents setPosition={setPosition} />
          </MapContainer>
        </div>
        <div className="flex justify-end gap-3">
          <button onClick={onClose} className="px-4 py-2 text-slate-600 font-bold">Cancel</button>
          <button onClick={handleConfirm} className="px-4 py-2 bg-ondc-blue text-white rounded-lg font-bold">Confirm</button>
        </div>
      </div>
    </div>
  );
}
