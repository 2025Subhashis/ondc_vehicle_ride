import { useState, useEffect } from 'react';
import { MapContainer, TileLayer, Marker, useMap, useMapEvents } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import L, { LatLng } from 'leaflet';
// @ts-ignore
import { GeoSearchControl, OpenStreetMapProvider } from 'leaflet-geosearch';
import 'leaflet-geosearch/dist/geosearch.css';

// Fix for default marker icon
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
  onLocationSelect: (label: string, latlng: string) => void;
  onClose: () => void;
}

function SearchControl() {
  const map = useMap();
  useEffect(() => {
    const provider = new OpenStreetMapProvider();
    const searchControl = new (GeoSearchControl as any)({
      provider: provider,
      style: 'bar',
      autoComplete: true,
      autoCompleteDelay: 250,
      showMarker: false,
    });
    map.addControl(searchControl);
    return () => { map.removeControl(searchControl); };
  }, [map]);
  return null;
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
  const [address, setAddress] = useState<string>('');

  const reverseGeocode = async (lat: number, lng: number) => {
    const url = `https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lng}`;
    try {
      const response = await fetch(url);
      const data = await response.json();
      setAddress(data.display_name || 'Unknown Location');
    } catch {
      setAddress(`${lat}, ${lng}`);
    }
  };

  const handleSetPosition = (pos: LatLng) => {
    setPosition(pos);
    reverseGeocode(pos.lat, pos.lng);
  }

  const handleConfirm = () => {
    if (position) {
      onLocationSelect(address, `${position.lat},${position.lng}`);
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div className="bg-white w-full max-w-lg p-6 rounded-3xl shadow-2xl">
        <h3 className="text-xl font-bold mb-4">Select Location</h3>
        <div className="h-64 mb-4 rounded-xl overflow-hidden relative">
          <MapContainer center={[12.9716, 77.5946] as L.LatLngExpression} zoom={13} style={{ height: '100%', width: '100%' }}>
            <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
            <SearchControl />
            {position && <Marker position={position} />}
            <MapEvents setPosition={handleSetPosition} />
          </MapContainer>
        </div>
        <p className="text-sm text-slate-600 mb-4 truncate">Selected: {address || 'None'}</p>
        <div className="flex justify-end gap-3">
          <button onClick={onClose} className="px-4 py-2 text-slate-600 font-bold">Cancel</button>
          <button onClick={handleConfirm} disabled={!position} className="px-4 py-2 bg-ondc-blue text-white rounded-lg font-bold disabled:opacity-50">Confirm</button>
        </div>
      </div>
    </div>
  );
}
