import { useState } from 'react';

interface IssueReportProps {
  transactionId: string;
  onClose: () => void;
}

export default function IssueReport({ transactionId, onClose }: IssueReportProps) {
  const [shortDesc, setShortDesc] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    
    // Construct ONDC-compliant Issue Message
    const issuePayload = {
      context: {
        action: "issue",
        transaction_id: transactionId,
        timestamp: new Date().toISOString(),
      },
      message: {
        issue: {
          id: `issue_${Date.now()}`,
          category: "RIDE",
          sub_category: "CANCELLED",
          status: "OPEN",
          description: {
            short_desc: shortDesc,
            long_desc: "User reported an issue: " + shortDesc
          }
        }
      }
    };

    try {
      await fetch('https://ondcvehicleride-production.up.railway.app/issue', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(issuePayload)
      });
      alert('Issue reported successfully!');
      onClose();
    } catch (error) {
      console.error('Failed to report issue:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div className="bg-white w-full max-w-md p-6 rounded-3xl shadow-2xl">
        <h3 className="text-xl font-bold mb-4">Report an Issue</h3>
        <form onSubmit={handleSubmit}>
          <textarea 
            value={shortDesc}
            onChange={(e) => setShortDesc(e.target.value)}
            placeholder="Briefly describe the issue..."
            className="w-full p-4 bg-slate-50 rounded-xl mb-4"
            required
          />
          <div className="flex justify-end gap-3">
            <button type="button" onClick={onClose} className="px-4 py-2 text-slate-600 font-bold">Cancel</button>
            <button type="submit" disabled={loading} className="px-4 py-2 bg-red-600 text-white rounded-lg font-bold">
              {loading ? 'Submitting...' : 'Submit Issue'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
