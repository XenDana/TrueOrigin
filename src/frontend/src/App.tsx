import './App.css';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import TestPage from './pages/test-page';
import Homepage from './pages/home';
import Dashboard from './pages/dashboard';

function App() {

  return (
    <>
      <Router>
        <Routes>
            <Route path="/" element={<Homepage />} />
            <Route path="/test" element={<TestPage />} />
            <Route path="/dashboard" element={<Dashboard />} />
          </Routes>
      </Router>
    </>
  )
}

export default App
