import './App.css';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import TestPage from './pages/test-page';
import Homepage from './pages/home';
import Dashboard from './pages/dashboard';
import LoginPage from './pages/auth/login';
import MobileScan from './pages/mobile-scan';
import MobileSuccess from './pages/mobile-success';
import MobileFailed from './pages/mobile-failed';
import MobileCoinGranted from './pages/mobile-coin-granted';

function App() {
  return (
    <Router>
      <Routes>
          <Route path="/" element={<Homepage />} />
          <Route path="/auth/login" element={<LoginPage />} />
          <Route path="/test" element={<TestPage />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/mobile" element={<MobileScan />} />
          <Route path="/mobile-success" element={<MobileSuccess />} />
          <Route path="/mobile-failed" element={<MobileFailed />} />
          <Route path="/mobile-coin-granted" element={<MobileCoinGranted />} />
        </Routes>
    </Router>
  )
}

export default App
