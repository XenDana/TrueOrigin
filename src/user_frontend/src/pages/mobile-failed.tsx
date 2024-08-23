import logo from "../assets/true-origin.png"
import warningImageUrl from "../assets/scan-success-red.png"

const MobileFailed: React.FC = () => {
    return (
        <div className="flex justify-center items-center min-h-screen bg-gray-100 p-4">
          <div className="w-full max-w-md bg-white p-6 rounded-lg shadow-md text-center">
            {/* Logo */}
            <img src={logo} alt="TrueOrigin Logo" className="mt-4 mb-6" />
    
            {/* Scan Success Message */}
            <h1 className="text-2xl font-semibold mb-4">Scan Success!</h1>
            
            {/* Warning Image */}
            <div className="mb-4">
              <img src={warningImageUrl} alt="Warning" className="mx-auto" />
            </div>
    
            {/* Product Verified */}
            <div className="mb-4">
              <span className="inline-block bg-red-500 text-white rounded-full px-4 py-1 text-sm font-semibold">
                Product Verified
              </span>
            </div>
    
            {/* Product Scanned Message */}
            <h2 className="text-xl font-semibold mb-2">Product is already scanned more than 1 time!</h2>
            
            {/* Additional Text */}
            <p className="text-gray-600 mb-6">Please beware!</p>
            
            {/* CTA Button */}
            <button className="bg-red-500 text-white px-6 py-3 rounded-full hover:bg-red-600 transition">
              Sorry, You are not eligible
            </button>
          </div>
        </div>
      );
}

export default MobileFailed;
