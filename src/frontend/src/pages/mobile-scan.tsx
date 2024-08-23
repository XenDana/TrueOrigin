import React from 'react';
import { useState } from 'react';

import { QrReader } from 'react-qr-reader';

import logo from "../assets/true-origin.png"

const Mobile = () => {
    const [scanResult, setScanResult] = useState<string>('');

    const handleScan = (data: any) => {
        if (data && data.text) {
            console.log("data ", data.text);
            setScanResult(data.text);
        }
    };
  
    const handleError = (err: any) => {
      console.error(err);
    };

    return (
        <>
            <div className="bg-gray-100 flex justify-center items-center min-h-screen p-4">
                <div className="w-full max-w-md">
                    <div className="bg-white p-4 rounded-lg shadow-md flex flex-col items-center">
                    <img src={logo} alt="TrueOrigin Logo" className="mt-4 mb-6"/>
                    
                    <h2 className="text-center text-lg font-semibold mb-4">Scan Here</h2>
                    
                    <div className="w-full p-4 rounded-card mb-4">
                        {scanResult == '' && <QrReader
                            delay={300}
                            onError={handleError}
                            onResult={handleScan}
                            style={{ width: '100%' }}
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
                    
                    <button className="bg-gray-800 text-white px-6 py-3 rounded-full flex items-center hover:bg-gray-700">
                        Scan
                        {/* <svg className="ml-2 animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                        <path d="M4 12a8 8 0 018-8" strokeLinecap="round" strokeLinejoin="round" strokeWidth="4"></path>
                        </svg> */}
                    </button>
                    </div>
                </div>
            </div>
        </>
    )
}

export default Mobile;
