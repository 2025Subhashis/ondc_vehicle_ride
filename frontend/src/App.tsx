import { useState } from 'react'

interface RideProvider {
  id: string;
  descriptor: { name: string };
  items: RideItem[];
}

interface RideItem {
  id: string;
  descriptor: { name: string };
  price: { value: string; currency: string };
}

function App() {
  const [pickup, setPickup] = useState('')
  const [drop, setDrop] = useState('')
  const [loading, setLoading] = useState(false)
  const [results, setResults] = useState<RideProvider[]>([])

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    try {
      const response = await fetch('http://127.0.0.1:8080/search', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ pickup_location: pickup, drop_location: drop })
      })
      const data = await response.json()
      if (data.message && data.message.catalog) {
        setResults(data.message.catalog.providers)
      }
    } catch (error) {
      console.error('Search failed:', error)
      // Fallback mock data for demo if backend is unreachable
      setResults([{
        id: 'mock_bpp',
        descriptor: { name: 'Demo Provider' },
        items: [{
          id: 'mock_item',
          descriptor: { name: 'Standard Cab' },
          price: { value: '150.0', currency: 'INR' }
        }]
      }])
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-slate-50">
      {/* Header */}
      <header className="bg-white border-b px-6 py-4 sticky top-0 z-10">
        <div className="max-w-6xl mx-auto flex justify-between items-center">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 bg-ondc-blue rounded-lg flex items-center justify-center text-white font-bold">V</div>
            <h1 className="text-xl font-bold text-slate-900 tracking-tight">ONDC Vehicle</h1>
          </div>
          <nav className="flex gap-6 text-sm font-medium text-slate-600">
            <a href="#" className="hover:text-ondc-blue transition-colors">My Bookings</a>
            <a href="#" className="hover:text-ondc-blue transition-colors">Support</a>
            <button className="bg-ondc-blue text-white px-4 py-2 rounded-full hover:bg-ondc-dark transition-all">Login</button>
          </nav>
        </div>
      </header>

      <main className="max-w-6xl mx-auto px-6 py-12">
        {/* Hero & Search */}
        <div className="grid lg:grid-cols-2 gap-12 items-center mb-16">
          <div>
            <h2 className="text-5xl font-extrabold text-slate-900 leading-tight mb-6">
              Your next ride is <br/>
              <span className="text-ondc-blue">just a click away.</span>
            </h2>
            <p className="text-lg text-slate-600 mb-8 max-w-md">
              The open network for mobility. Transparent pricing, reliable providers, and seamless booking.
            </p>
            
            <form onSubmit={handleSearch} className="bg-white p-6 rounded-3xl shadow-xl shadow-slate-200 border border-slate-100 space-y-4">
              <div className="space-y-1">
                <label className="text-xs font-bold text-slate-400 uppercase ml-1">Pickup Location</label>
                <input 
                  type="text" 
                  value={pickup}
                  onChange={(e) => setPickup(e.target.value)}
                  placeholder="Where from?" 
                  className="w-full px-4 py-3 bg-slate-50 border-none rounded-2xl focus:ring-2 focus:ring-ondc-blue text-slate-900"
                  required
                />
              </div>
              <div className="space-y-1">
                <label className="text-xs font-bold text-slate-400 uppercase ml-1">Drop Location</label>
                <input 
                  type="text" 
                  value={drop}
                  onChange={(e) => setDrop(e.target.value)}
                  placeholder="Where to?" 
                  className="w-full px-4 py-3 bg-slate-50 border-none rounded-2xl focus:ring-2 focus:ring-ondc-blue text-slate-900"
                  required
                />
              </div>
              <button 
                disabled={loading}
                className="w-full bg-ondc-blue text-white py-4 rounded-2xl font-bold text-lg hover:shadow-lg hover:shadow-blue-200 transition-all active:scale-95 disabled:opacity-50"
              >
                {loading ? 'Finding rides...' : 'Search Rides'}
              </button>
            </form>
          </div>
          
          <div className="hidden lg:block">
             <div className="relative">
                <div className="absolute -inset-4 bg-ondc-blue/10 rounded-full blur-3xl"></div>
                <img 
                  src="/hero-cab.png" 
                  alt="Cab Illustration" 
                  className="relative w-full drop-shadow-2xl"
                  onError={(e) => { e.currentTarget.src = "https://img.freepik.com/free-vector/isometric-taxi-cab-illustration_23-2148281358.jpg?w=826&t=st=1716572500~exp=1716573100~hmac=5b47a9d0f9f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3" }}
                />
             </div>
          </div>
        </div>

        {/* Results */}
        {results.length > 0 && (
          <section className="space-y-6">
            <h3 className="text-2xl font-bold text-slate-900">Available Rides</h3>
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {results.map((provider) => (
                provider.items.map((item) => (
                  <div key={item.id} className="bg-white p-6 rounded-3xl border border-slate-100 shadow-sm hover:shadow-md transition-shadow group">
                    <div className="flex justify-between items-start mb-4">
                      <div>
                        <span className="text-xs font-bold text-ondc-blue bg-ondc-light px-2 py-1 rounded-md mb-2 inline-block">
                          {provider.descriptor.name}
                        </span>
                        <h4 className="text-lg font-bold text-slate-900">{item.descriptor.name}</h4>
                      </div>
                      <div className="text-right">
                        <span className="text-2xl font-black text-slate-900">₹{item.price.value}</span>
                        <p className="text-xs text-slate-400 font-medium">Est. Fare</p>
                      </div>
                    </div>
                    <button className="w-full py-3 bg-slate-900 text-white rounded-xl font-bold group-hover:bg-ondc-blue transition-colors">
                      Book Now
                    </button>
                  </div>
                ))
              ))}
            </div>
          </section>
        )}
      </main>

      {/* Footer */}
      <footer className="bg-slate-900 text-slate-400 py-12 px-6">
        <div className="max-w-6xl mx-auto flex flex-col md:flex-row justify-between items-center gap-8">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 bg-slate-700 rounded flex items-center justify-center text-white text-xs font-bold">V</div>
            <span className="text-white font-bold">ONDC Vehicle Booking</span>
          </div>
          <div className="flex gap-8 text-sm">
            <a href="#" className="hover:text-white">Privacy</a>
            <a href="#" className="hover:text-white">Terms</a>
            <a href="#" className="hover:text-white">Contact</a>
          </div>
          <p className="text-sm">© 2026 ONDC Mobility Protocol Demo</p>
        </div>
      </footer>
    </div>
  )
}

export default App
