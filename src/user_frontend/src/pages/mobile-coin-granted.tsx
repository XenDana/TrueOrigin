import logo from "../assets/true-origin.png"
import coinGrantedImage from '../assets/coin-granted.png'

const MobileCoinGranted: React.FC = () => {
    return (
        <div className="flex justify-center items-center min-h-screen p-4">
          <div className="w-full max-w-md p-6 rounded-lg shadow-md text-center">
            {/* Logo */}
            <img src={logo} alt="TrueOrigin Logo" className="mt-4 mb-6" />
            
            {/* Scan Success Message */}
            <h1 className="text-2xl font-semibold mb-4">Scan Success!</h1>
            
            {/* Success Image */}
            <div className="mb-4">
              <img src={coinGrantedImage} alt="Coin Sent" className="mx-auto" />
            </div>
            
            {/* Coin Sent Message */}
            <h2 className="text-xl font-semibold mb-2">Coin Sent! Please Check Your Wallet</h2>
            
            {/* Additional Text */}
            <p className="text-gray-600 mb-6">Happy Shopping!</p>
            
            {/* CTA Button */}
            <button className="bg-green-500 text-white px-6 py-3 rounded-full hover:bg-green-600 transition">
              Back to Home &rarr;
            </button>
          </div>
        </div>
      );
}

export default MobileCoinGranted;
