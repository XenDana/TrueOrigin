import logo from "../assets/true-origin.png"
import successImageUrl from "../assets/scan-success.png"

const MobileSuccess: React.FC = () => {
  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-100 p-4">
      <div className="w-full max-w-md bg-white p-6 rounded-lg shadow-md text-center items-center justify-center">
        {/* Logo */}
        <img src={logo} alt="TrueOrigin Logo" className="mt-4 mb-6"/>
        
        {/* Scan Success Message */}
        <h1 className="text-2xl font-semibold mb-4">Scan Success!</h1>
        
        {/* Success Image */}
        <div className="mb-4">
          <img src={successImageUrl} alt="Product Verified" className="mx-auto" />
        </div>
        
        {/* Product Verified */}
        <div className="mb-4">
          <span className="inline-block bg-yellow-500 text-white rounded-full px-4 py-1 text-sm font-semibold">
            Product Verified
          </span>
        </div>

        {/* 1st Time Scan */}
        <h2 className="text-xl font-semibold mb-2">1st Time Scan!</h2>
        
        {/* Additional Text */}
        <p className="text-gray-600 mb-6">Your Product is Genuine!</p>
        
        {/* CTA Button */}
        <button className="bg-green-500 text-white px-6 py-3 rounded-full hover:bg-green-600 transition">
          Click to Redeem Coin &rarr;
        </button>
      </div>
    </div>
  );
}

export default MobileSuccess;
