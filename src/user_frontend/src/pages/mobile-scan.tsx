import { useState } from 'react';

import { QrReader } from 'react-qr-reader';

import logo from "../assets/true-origin.png";

interface MobileProps {
    scanResultFunc: Function; // Optional prop
}

const Mobile: React.FC<MobileProps> = ({ scanResultFunc }) => {
    const [scanResult, setScanResult] = useState<string>('');

    const handleScan = (data: any) => {
        console.log("data scanned ", data);
        if (data && data.text) {
            if (data.text !== '') {
                setScanResult(data.text);
                scanResultFunc(data.text);
            }
        }
    };

    return (
        <>
            <div className="flex justify-center items-center min-h-screen p-4">
                <div className="w-full max-w-md">
                    <div className="p-4 rounded-lg shadow-md flex flex-col items-center">
                        <img src={logo} alt="TrueOrigin Logo" className="mt-4 mb-6"/>
                        
                        <h2 className="text-center text-lg font-semibold mb-4">Scan Here</h2>
                        
                        <div className="w-full p-4 rounded-card mb-4">
                            {scanResult == '' && <QrReader
                                scanDelay={300}
                                onResult={handleScan}
                                constraints={{ facingMode: 'user' }}
                            />}
                        </div>
                        
                        <p className="text-center text-sm text-gray-600 mb-6">
                            Maybe have some hint about our True Origin, so user can understand more about it and this is just dummy text but cooler way, not like lorem ipsum one haha.
                        </p>

                        {scanResult && (
                            <p className="text-center text-sm text-gray-600 mb-6">
                                Scanned QR Code Data: {scanResult}
                            </p>
                        )}
                    </div>
                </div>
            </div>
        </>
    )
}

export default Mobile;
