import EditProfile from "./components/profile/EditProfile";
import Header from "./components/header/Header";
// import { NoProfileMessage } from "./components/profile/NoProfileMessage";
import Mobile from './pages/mobile-scan';
import MobileSuccess from './pages/mobile-success';
import MobileSuccessRed from './pages/mobile-failed';
import MobileCoinGranted from './pages/mobile-coin-granted';

import { useEffect, useState } from 'react'

function App() {
  const [scanResult, setScanResult] = useState('');
  const [content, setContent] = useState('');

  const handleScanResult = (scanResult: String) => {
    // hit backend
    // load new component

    console.log("handle scan result ", scanResult);

    if (scanResult === "http://example.com") {
      // load success
      setContent('success');
    } else if (scanResult === "http://xendana.com") {
      setContent('success-red');
    }
  }

  const showContent = () => {
    console.log("content on show content ", content);
    switch (content) {
      case 'scan': {
        return (
          <Mobile scanResultFunc={setScanResult}/>
        )
      } case 'success': {
        return (
          <MobileSuccess redeemCoinFunc={setContent}/>
        )
      } case 'granted': {
        return (
          <MobileCoinGranted backToHomeFunc={setContent} />
        )
      } case 'success-red': {
        return (
          <MobileSuccessRed />
        )
      } default:
        return <Mobile scanResultFunc={setScanResult}/>
    }
  }

  useEffect(() => {
    handleScanResult(scanResult)
  }, [scanResult])

  return (
    <div className="flex flex-col items-center w-full">
      <Header />
      <div className="flex flex-col items-center w-full gap-10 p-5">
        <div className="h-5 md:h-28" />
        {/* <NoProfileMessage /> */}
        <EditProfile className="w-full max-w-2xl border-zinc-700/50 border-[1px] bg-zinc-900 drop-shadow-xl rounded-3xl flex flex-col items-center px-5 md:px-24 py-8" />
        { showContent() }
      </div>
    </div>
  );
}

export default App;
